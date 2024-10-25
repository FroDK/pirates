use std::borrow::BorrowMut;
use std::process::{Child, Command};

pub struct APIManager {
    cmd: Command,
    child: Option<Child>,
    // api_process: Option<GroupChild>,
}

impl APIManager {
    pub fn new() -> Result<APIManager, Box<dyn std::error::Error>> {
        println!("Текущая директория: {:?}", std::env::current_dir().unwrap());

        let exe_path = std::env::current_dir().unwrap().join("resources").join("websocket-server.exe");
        let exe_path_str = exe_path.to_str().unwrap();
        println!("Путь к исполняемому файлу: {}", exe_path_str);
        
        let tt = Command::new(exe_path_str);

        
        Ok(APIManager {
            cmd: tt,
            child: None,
        })
    }

    pub fn start_backend(&mut self) -> Result<String, String> {
        match self.child.borrow_mut() {
            Some(_) => {
                let info =
                    "Дочерний процесс службы API не является пустым и больше не будет создан";
                println!("{}", &info);
                Ok(info.into())
            }
            None => {
                let child = self.cmd.spawn();
                match child {
                    Ok(v) => {
                        self.child = Some(v);
                        let info = "успешный запуск api";
                        println!("{}", &info);
                        Ok(info.into())
                    }
                    Err(_) => {
                        let info = "не удалось запустить api";
                        println!("{}", &info);
                        println!("{}", &child.err().unwrap());
                        Err(info.into())
                    }
                }
            }
        }
    }

    pub fn terminate_backend(&mut self) -> Result<String, String> {
        match self.child.borrow_mut() {
            Some(child) => {
                // child.wait().expect("Some error happened when killing child process");
                child
                    .kill()
                    .expect("При уничтожении дочернего процесса произошла какая-то ошибка");
                self.child = None;
                let info = "Убейте уже существующий дочерний процесс, а затем установите для него значение Нет";
                println!("{}", &info);
                Ok(info.into())
            }
            _ => {
                let info = "Дочерний процесс API в настоящее время не существует, никаких операций не требуется";
                println!("{}", &info);
                Ok(info.into())
            }
        }
    }

    pub fn restart_backend(&mut self) -> Result<String, String> {
        let terminate_result = self.terminate_backend();
        match terminate_result {
            Ok(_) => {
                println!("Было выполнено действие по завершению работы API");
                match self.start_backend() {
                    Ok(_) => {
                        let info = "Успешно перезапущен сервер API";
                        println!("{}", &info);
                        Ok(info.into())
                    }
                    Err(e) => {
                        println!("{}", &e);
                        return Err(e.into());
                    }
                }
            }
            Err(e) => {
                println!("{}", &e);
                return Err(e);
            }
        }
    }
}
