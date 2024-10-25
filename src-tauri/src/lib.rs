#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod api_manager;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use serde_json::Value;
use std::sync::Arc;
use tauri::Manager;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

// struct APIManagerState {
//     api_manager_mutex: Mutex<APIManager>,
// }

// #[tauri::command]
// fn start_server(api_manager_state: State<APIManagerState>) -> Result<String, String> {
//     let am = api_manager_state
//         .api_manager_mutex
//         .lock()
//         .unwrap()
//         .start_backend();
//     am
// }

// #[tauri::command]
// fn stop_server(api_manager_state: State<APIManagerState>) -> Result<String, String> {
//     let am = api_manager_state
//         .api_manager_mutex
//         .lock()
//         .unwrap()
//         .terminate_backend();
//     am
// }

// #[tauri::command]
// fn restart_server(api_manager_state: State<APIManagerState>) -> Result<String, String> {
//     let am = api_manager_state
//         .api_manager_mutex
//         .lock()
//         .unwrap()
//         .restart_backend();
//     am
// }

type SharedSink =
    Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>;
type SharedClients = Arc<Mutex<Vec<SharedSink>>>;

#[tokio::main]
pub async fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let clients: SharedClients = Arc::new(Mutex::new(Vec::new()));
            let clients_clone = clients.clone();

            tokio::spawn(async move {
                let listener = TcpListener::bind("0.0.0.0:9001").await.unwrap();
                println!("WebSocket сервер запущен на 0.0.0.0:9001");

                while let Ok((stream, addr)) = listener.accept().await {
                    println!("Новое подключение: {}", addr);
                    let ws_stream = accept_async(stream).await.unwrap();
                    let (write, mut read) = ws_stream.split();
                    let write = Arc::new(Mutex::new(write));
                    let clients_for_loop = clients_clone.clone();

                    {
                        let mut clients = clients_for_loop.lock().await;
                        clients.push(write.clone());
                        println!(
                            "Клиент добавлен. Текущее количество клиентов: {}",
                            clients.len()
                        );
                    }

                    let clients_for_read = clients_for_loop.clone();

                    tokio::spawn(async move {
                        while let Some(Ok(message)) = read.next().await {
                            if let Ok(text) = message.to_text() {
                                println!("Получено сообщение: {}", text);
                                if let Ok(json) = serde_json::from_str::<Value>(text) {
                                    if json["type"] == "update" {
                                        println!("Обнаружено сообщение типа 'update'");
                                        let mut clients = clients_for_read.lock().await;
                                        println!(
                                            "Количество подключенных клиентов: {}",
                                            clients.len()
                                        );
                                        for client in clients.iter_mut() {
                                            let mut unlocked_client = client.lock().await;
                                            match unlocked_client
                                                .send(Message::Text(text.to_string()))
                                                .await
                                            {
                                                Ok(_) => {
                                                    println!("Сообщение успешно отправлено клиенту")
                                                }
                                                Err(e) => println!(
                                                    "Ошибка при отправке сообщения клиенту: {:?}",
                                                    e
                                                ),
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        println!("Соединение закрыто, добавляем клиента в список");
                        let mut clients_for_push = clients_for_read.lock().await;
                        clients_for_push.push(write.clone());
                        println!(
                            "Клиент добавлен. Текущее количество клиентов: {}",
                            clients_for_push.len()
                        );
                    });
                }
            });

            app.manage(clients);
            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("ошибка при запуске приложения tauri");
}
