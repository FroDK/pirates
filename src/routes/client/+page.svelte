<script lang="ts">
  import { AspectRatio } from "$lib/components/ui/aspect-ratio/index.js";
  import * as Resizable from "$lib/components/ui/resizable";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import {
    CalendarDays,
    Sailboat,
    Sword,
    TentTree,
    Shield,
    Loader,
  } from "lucide-svelte";
  import { Separator } from "@/components/ui/separator";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Input } from "$lib/components/ui/input";
  import { locations, weatherData } from "../game/weather_data";
  import { onDestroy } from "svelte";

  interface IOtherObjects {
    id: string;
    x: number;
    y: number;
    width: number;
    height: number;
    path: string;
    onPositionChange?: (id: string, x: number, y: number) => void;
  }

  let ws: any;

  import { page } from "$app/stores";

  let ip = "";
  let port = 0;

  const urlParams = new URLSearchParams($page.url.searchParams);
  ip = urlParams.get("ip") || "";
  port = parseInt(urlParams.get("port") || "0", 10);

  console.log("IP:", ip);
  console.log("Порт:", port);

  let sizeMap: Record<number, { width: number; height: number }> = {
    0: {
      width: 44,
      height: 55,
    },
    1: {
      width: 55,
      height: 68,
    },
    2: {
      width: 88,
      height: 110,
    },
    3: {
      width: 110,
      height: 137,
    },
    4: {
      width: 137,
      height: 171,
    },
  };

  interface IData {
    ships: IShip[];
    expedition: number;
    day: number;
    attackingShip: IShip | null;
    defendingShip: IShip | null;
    weather: string[];
    selectedWeaponAttackingShip: string;
    selectedWeaponDefendingShip: string;
    otherObjects: IOtherObjects[];
  }

  let data: IData = $state({
    ships: [],
    expedition: 1,
    day: 1,
    attackingShip: null,
    defendingShip: null,
    weather: [],
    selectedWeaponAttackingShip: "",
    selectedWeaponDefendingShip: "",
    otherObjects: [],
  });

  let showAlertErrorConnect = $state(false);

  async function connectWebSocket() {
    try {
      ws = new WebSocket(`ws://${ip}:${port}/ws`);

      ws.onopen = function (event: any) {
        console.log("Подключено к WebSocket");
        isGameReady = true;
      };

      ws.onerror = function (event: any) {
        console.error("Ошибка при подключении к WebSocket:", event);
        showAlertErrorConnect = true;
      };

      ws.onmessage = (event: MessageEvent) => {
        console.log("Получено сообщение:", event.data);
        // Здесь вы можете обрабатывать входящие сообщения
        try {
          const parsedMsg = JSON.parse(event.data);
          handleIncomingMessage(parsedMsg);
        } catch (error) {
          console.error("Ошибка при разборе входящего сообщения:", error);
        }
      };
    } catch (error) {
      console.error("Ошибка при подключении к WebSocket:", error);
    }
  }

  function handleIncomingMessage(message: any) {
    // Обработка входящих сообщений
    switch (message.type) {
      case "update":
        console.log("Получено обновление:", message.data);
        data = message.data;
        // Здесь вы можете обновить состояние вашего приложения
        break;
      // Добавьте другие типы сообщений по мере необходимости
      default:
        console.log("Получено неизвестное сообщение:", message);
    }
  }

  async function disconnectWebSocket() {
    if (ws) {
      try {
        console.log("WebSocket:", ws);

        if (ws.readyState === WebSocket.OPEN) {
          await ws.close();

          console.log("WebSocket отключен");
        }
      } catch (error) {
        console.error("Ошибка при отключении WebSocket:", error);
      }
    }
  }

  function redirectToMainPage() {
    window.location.href = "/";
  }

  let isGameReady = $state(false);

  // Подключаемся к WebSocket при инициализации компонента
  connectWebSocket();

  onDestroy(() => {
    disconnectWebSocket();
  });

  interface IShip {
    id: string;
    isAttacking: boolean;
    isDefending: boolean;
    color: string;
    number: number;
    x: number;
    y: number;
    width: number;
    height: number;
  }
</script>

<AlertDialog.Root bind:open={showAlertErrorConnect}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>Ошибка подключения</AlertDialog.Title>
      <AlertDialog.Description>
        Не удалось подключиться к серверу. Пожалуйста, убедитесь, что IP и порт
        указаны правильно.
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel onclick={redirectToMainPage}>
        Вернуться на главную
      </AlertDialog.Cancel>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>

{#if isGameReady}
  <div class="game-container">
    <nav>
      <div class="expedition nav-item">
        <TentTree color="#616161" class="mr-2" />
        <span class="text-lg text-slate-700">Экспедиция: {data.expedition}</span
        >
      </div>
      <div class="day nav-item">
        <CalendarDays color="#616161" class="mr-2" /><span
          class="text-lg text-slate-700">День: {data.day}</span
        >
      </div>

      <Separator orientation="vertical" />

      <div class="weather nav-item">
        {#each locations as location, index}
          <div class="weather-item">
            <div class="text-lg text-slate-700 font-semibold">
              {location.label}:
            </div>
            <div>
              <img
                src={`/weather/${weatherData[data.expedition - 1][data.day - 1][index]}.png`}
                alt={location.label}
                height="40"
                width="40"
              />
            </div>
          </div>
        {/each}
      </div>
    </nav>

    <div class="game-area">
      <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={80}>
          <div class="map-container">
            <AspectRatio ratio={16 / 9}>
              <img
                src="/map.png"
                alt="Gray by Drew Beamer"
                class="h-full w-full rounded-xl"
              />
              <img
                src="/logo.png"
                alt="logo"
                class="absolute bottom-0 right-0 w-[165px] h-[100px]"
              />

              {#each data.ships as ship (ship.id)}
                <img
                  src={`/gifs/${ship.number}_${ship.color}.gif`}
                  alt="ship"
                  class="absolute"
                  style={`left: calc(${ship.x}% - ${ship.width / 2}px); top: calc(${ship.y}% - ${ship.height / 2}px); width: ${ship.width}px; height: ${ship.height}px;`}
                />
              {/each}

              {#each data.otherObjects as otherObject}
                <img
                  src={otherObject.path}
                  alt="other-object"
                  class="absolute"
                  style={`left: calc(${otherObject.x}% - ${otherObject.width / 2}px); top: calc(${otherObject.y}% - ${otherObject.height / 2}px); width: ${otherObject.width}px; height: ${otherObject.height}px;`}
                />
              {/each}
            </AspectRatio>
          </div>
        </Resizable.Pane>

        <Resizable.Handle withHandle />

        <Resizable.Pane>
          <div class="side-panel">
            <div class="settings">
              <Card.Root
                class="w-full bg-gradient-to-b from-white to-gray-50 shadow-md duration-300 rounded-xl"
              >
                <Card.Header>
                  <Card.Title>Сражение</Card.Title>
                </Card.Header>
                <Card.Content>
                  <div class="flex gap-4 relative justify-between">
                    <div class="flex flex-col gap-4">
                      <div class="m-auto relative">
                        {#if data.attackingShip}
                          <img
                            src={`/gifs/${data.attackingShip.number}_${data.attackingShip.color}.gif`}
                            alt="ship"
                            height="88"
                            width="88"
                          />
                        {:else}
                          <div
                            class="w-[100px] h-[100px] bg-[#f0f0f0] flex justify-center items-center"
                          >
                            <Sailboat size={48} color="#6b6b6b" />
                          </div>
                        {/if}
                      </div>
                      <div class="weapon-selector">
                        <div class="flex flex-col items-center space-y-1.5">
                          <Label for="weapon" class="flex items-center"
                            ><Sword class="mr-2 h-4 w-4" /><span>Оружие</span
                            ></Label
                          >
                          <Input
                            id="weapon"
                            readonly
                            class="cursor-not-allowed focus-visible:ring-opacity-0"
                            bind:value={data.selectedWeaponAttackingShip}
                          />
                        </div>
                      </div>
                    </div>

                    {#if data.attackingShip && data.defendingShip}
                      <img
                        src="/swords.gif"
                        alt="swords"
                        height="100"
                        width="100"
                        class="absolute top-0 left-1/2 transform -translate-x-1/2"
                      />
                    {/if}

                    <div class="flex flex-col gap-4">
                      <div class="m-auto relative">
                        {#if data.defendingShip}
                          <img
                            src={`/gifs/${data.defendingShip.number}_${data.defendingShip.color}.gif`}
                            alt="ship"
                            height="88"
                            width="88"
                          />
                        {:else}
                          <div
                            class="w-[100px] h-[100px] bg-[#f0f0f0] flex justify-center items-center"
                          >
                            <Sailboat size={48} color="#6b6b6b" />
                          </div>
                        {/if}
                      </div>
                      <div class="weapon-selector">
                        <div class="flex flex-col space-y-1.5 items-center">
                          <Label for="weapon" class="flex items-center"
                            ><Shield class="mr-2 h-4 w-4" /><span>
                              Броня</span
                            ></Label
                          >
                          <Input
                            id="weapon"
                            readonly
                            class="cursor-not-allowed focus-visible:ring-opacity-0"
                            bind:value={data.selectedWeaponDefendingShip}
                          />
                        </div>
                      </div>
                    </div>
                  </div>
                </Card.Content>
              </Card.Root>
            </div>
          </div></Resizable.Pane
        >
      </Resizable.PaneGroup>
    </div>
  </div>
{:else}
  <div class="flex justify-center items-center h-screen">
    <div class="bg-white p-8 rounded-xl shadow-lg border border-gray-100">
      <Loader size={48} color="#007bff" class="animate-spin-slow" />
    </div>
  </div>
{/if}

<style>
  .nav-item {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .weather-item {
    word-wrap: break-word;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .weather {
    display: flex;
    gap: 20px;
  }

  .side-panel {
    display: flex;
    flex-direction: column;
    gap: 10px;
    height: 100%;
    padding: 0px 16px;
  }

  .settings {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: self-start;
    gap: 10px;
  }

  .game-container {
    margin: 0;
    width: 100%;
    height: 100vh;
    display: flex;
    flex-direction: column;
    gap: 20px;
    background-color: #f6f6f6;
  }

  nav {
    display: flex;
    justify-content: center;
    border: 1px solid #ccc;
    gap: 40px;
    padding: 10px 24px;
    min-height: 60px;
    border-radius: 10px;
    margin: 20px auto 0;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    background: linear-gradient(to bottom, #ffffff, #f5f5f5);
  }

  .game-area {
    display: flex;
    flex-grow: 1;
  }

  .map-container {
    flex: 1;
    padding: 0 20px;
  }
</style>
