<script lang="ts">
  import * as Select from "$lib/components/ui/select/index.js";
  import { Button } from "@/components/ui/button";
  import Ship from "./ship.svelte";
  import { weatherData, locations } from "./weather_data";
  import { AspectRatio } from "$lib/components/ui/aspect-ratio/index.js";
  import * as Resizable from "$lib/components/ui/resizable";
  import {
    CalendarDays,
    Sailboat,
    TentTree,
    X,
    Trash2,
    Sword,
    Shield,
  } from "lucide-svelte";
  import { Separator } from "@/components/ui/separator";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Input } from "$lib/components/ui/input";
  import { onDestroy } from "svelte";
  import { OtherObject } from "@/components/other-object";

  interface IOtherObjects {
    id: string;
    x: number;
    y: number;
    width: number;
    height: number;
    path: string;
    onPositionChange?: (id: string, x: number, y: number) => void;
  }

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

  let ships: IShip[] = $state([]);

  let otherObjects: IOtherObjects[] = $state([]);

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

  const numbers = [
    { value: 1, label: "1" },
    { value: 2, label: "2" },
    { value: 3, label: "3" },
    { value: 4, label: "4" },
    { value: 5, label: "5" },
  ];

  const colors = [
    { value: "blue", label: "Синий", hex: "#0078b3" },
    { value: "red", label: "Красный", hex: "#e31e24" },
    { value: "green", label: "Зеленый", hex: "#009846" },
    { value: "yellow", label: "Желтый", hex: "#fcc204" },
    { value: "violet", label: "Фиолетовый", hex: "#a79ecd" },
    { value: "orange", label: "Оранжевый", hex: "#ef7f1a" },
    { value: "lightblue", label: "Голубой", hex: "#a2d9f7" },
    { value: "rose", label: "Розовый", hex: "#f5b2b6" },
  ];

  const expeditions = Array.from({ length: weatherData.length }, (_, i) => ({
    value: i + 1,
    label: `Экспедиция ${i + 1}`,
  }));
  const days = Array.from({ length: weatherData[0].length }, (_, i) => ({
    value: i + 1,
    label: `День ${i + 1}`,
  }));

  let selectedColor = $state<(typeof colors)[0]>(colors[1]);
  let selectedNumber = $state<(typeof numbers)[0]>(numbers[1]);

  function addShip() {
    if (selectedColor.value && selectedNumber.value) {
      const newShip: IShip = {
        id: crypto.randomUUID(),
        color: selectedColor.value,
        number: selectedNumber.value,
        x: 50,
        y: 50,
        isAttacking: false,
        isDefending: false,
        width: sizeMap[2].width,
        height: sizeMap[2].height,
      };
      ships = [...ships, newShip];
    }
  }

  function removeShip(id: string) {
    ships = ships.filter((s) => s.id !== id);
  }

  let day = $state(days[0]);
  let expedition = $state(expeditions[0]);

  let selectedWeaponAttackingShip = $state<string>("");
  let attackingShip = $state<IShip | null>(null);

  let selectedWeaponDefendingShip = $state<string>("");
  let defendingShip = $state<IShip | null>(null);

  function setAttackingShip(id: string) {
    const ship = ships.find((s) => s.id === id) || null;
    if (ship) {
      ship.isAttacking = true;
      ship.isDefending = false;

      attackingShip = ship;
    }
  }

  function setDefendingShip(id: string) {
    const ship = ships.find((s) => s.id === id) || null;
    if (ship) {
      ship.isDefending = true;
      ship.isAttacking = false;
      defendingShip = ship;
    }
  }

  function resetAttackingShip() {
    const ship = ships.find((s) => s.id === attackingShip?.id) || null;
    if (ship) {
      ship.isAttacking = false;
    }
    attackingShip = null;
  }

  function resetDefendingShip() {
    const ship = ships.find((s) => s.id === defendingShip?.id) || null;
    if (ship) {
      ship.isDefending = false;
    }
    defendingShip = null;
  }

  function deleteAllShips() {
    ships = [];
    otherObjects = [];
  }

  function previousDay() {
    if (day.value > 1) {
      day = days[day.value - 2];
    } else if (expedition.value > 1) {
      expedition = expeditions[expedition.value - 2];
      day = days[27];
    }
  }

  function nextDay() {
    if (day.value < 28) {
      day = days[day.value];
    } else if (expedition.value < 3) {
      expedition = expeditions[expedition.value];
      day = days[0];
    }
  }

  let disabledNextDay = $derived(day.value === 28 && expedition.value === 3);
  let disabledPreviousDay = $derived(day.value === 1 && expedition.value === 1);

  let ws: any;

  async function connectWebSocket() {
    try {
      console.log("connectWebSocket");
      // ws = await WebSocket.connect("ws://127.0.0.1:9001/ws");
      ws = new WebSocket("ws://127.0.0.1:9001/ws");
      console.log("ws", ws);
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

      console.log("WebSocket подключен");
    } catch (error) {
      console.error("Ошибка при подключении к WebSocket:", error);
    }
  }

  function handleIncomingMessage(message: any) {
    // Обработка входящих сообщений
    switch (message.type) {
      case "update":
        console.log("Получено обновление:", message.data);
        // Здесь вы можете обновить состояние вашего приложения
        break;
      // Добавьте другие типы сообщений по мере необходимости
      default:
        console.log("Получено неизвестное сообщение:", message);
    }
  }

  function sendUpdate(data: any) {
    if (ws) {
      try {
        ws.send(JSON.stringify({ type: "update", data }));
        console.log("Обновление отправлено:", data);
      } catch (error) {
        console.error("Ошибка при отправке обновления:", error);
      }
    } else {
      console.error("WebSocket не подключен");
    }
  }

  async function disconnectWebSocket() {
    if (ws) {
      try {
        await ws.disconnect();
        console.log("WebSocket отключен");
      } catch (error) {
        console.error("Ошибка при отключении WebSocket:", error);
      }
    }
  }

  function onColorChange(ship: IShip, color: string) {
    ship.color = color;
  }

  function onNumberChange(ship: IShip, number: number) {
    ship.number = number;
  }

  function onSizeChange(ship: IShip, width: number, height: number) {
    ship.width = width;
    ship.height = height;
  }

  function addOtherObject(type: "pirates" | "aborigines") {
    const newOtherObject = {
      id: crypto.randomUUID(),
      x: 50,
      y: 50,
      width: type === "pirates" ? 88 : 88,
      height: type === "pirates" ? 88 : 110,
      path: `/${type}.gif`,
    };

    otherObjects = [...otherObjects, newOtherObject];
  }

  function removeOtherObject(id: string) {
    otherObjects = otherObjects.filter((o) => o.id !== id);
  }

  // Подключаемся к WebSocket при инициализации компонента
  connectWebSocket();

  onDestroy(() => {
    disconnectWebSocket();
  });

  $effect(() => {
    console.log("effect");
    console.log("ships", ships);

    const dataToSend = {
      ships,
      attackingShip,
      defendingShip,
      expedition: expedition.value,
      day: day.value,
      weather: weatherData[expedition.value - 1][day.value - 1],
      selectedWeaponAttackingShip,
      selectedWeaponDefendingShip,
      otherObjects: otherObjects,
    };

    console.log("dataToSend", dataToSend);

    if (ws.readyState === WebSocket.OPEN) {
      sendUpdate(dataToSend);
    }
  });
</script>

<div class="game-container">
  <nav>
    <div class="expedition nav-item">
      <TentTree color="#616161" class="mr-2" />
      <span class="text-lg text-slate-700">Экспедиция: {expedition.value}</span>
    </div>
    <div class="day nav-item">
      <CalendarDays color="#616161" class="mr-2" /><span
        class="text-lg text-slate-700">День: {day.value}</span
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
              src={`/weather/${weatherData[expedition.value - 1][day.value - 1][index]}.png`}
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

            {#each ships as ship (ship.id)}
              <Ship
                {colors}
                {numbers}
                {ship}
                attackingShip={attackingShip?.id}
                defendingShip={defendingShip?.id}
                onRemove={() => removeShip(ship.id)}
                onPositionChange={(id, x, y) => {
                  ships = ships.map((s) => (s.id === id ? { ...s, x, y } : s));
                }}
                onSetAttackingShip={() => setAttackingShip(ship.id)}
                onSetDefendingShip={() => setDefendingShip(ship.id)}
                onColorChange={(color) => onColorChange(ship, color)}
                onNumberChange={(number) => onNumberChange(ship, number)}
                onSizeChange={(width, height) =>
                  onSizeChange(ship, width, height)}
              />
            {/each}

            {#each otherObjects as otherObject (otherObject.id)}
              <OtherObject
                id={otherObject.id}
                x={otherObject.x}
                y={otherObject.y}
                width={otherObject.width}
                height={otherObject.height}
                path={otherObject.path}
                onPositionChange={(id, x, y) => {
                  otherObjects = otherObjects.map((o) =>
                    o.id === id ? { ...o, x, y } : o
                  );
                }}
                onRemove={(id) => removeOtherObject(id)}
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
                      {#if attackingShip}
                        <img
                          src={`/gifs/${attackingShip.number}_${attackingShip.color}.gif`}
                          alt="ship"
                          height="88"
                          width="88"
                        />
                        <Button
                          variant="outline"
                          size="icon"
                          class="absolute top-0 right-0 h-6 w-6"
                          on:click={() => resetAttackingShip()}
                        >
                          <X size={16} color="#6b6b6b" />
                        </Button>
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
                          placeholder="1-3"
                          bind:value={selectedWeaponAttackingShip}
                        />
                      </div>
                    </div>
                  </div>

                  {#if attackingShip && defendingShip}
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
                      {#if defendingShip}
                        <img
                          src={`/gifs/${defendingShip.number}_${defendingShip.color}.gif`}
                          alt="ship"
                          height="88"
                          width="88"
                        />
                        <Button
                          variant="outline"
                          size="icon"
                          class="absolute top-0 right-0 h-6 w-6"
                          on:click={() => resetDefendingShip()}
                        >
                          <X size={16} color="#6b6b6b" />
                        </Button>
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
                          placeholder="1-3"
                          bind:value={selectedWeaponDefendingShip}
                        />
                      </div>
                    </div>
                  </div>
                </div>
              </Card.Content>
            </Card.Root>

            <Card.Root
              class="w-full bg-gradient-to-b from-white to-gray-50 shadow-md duration-300 rounded-xl"
            >
              <Card.Header>
                <Card.Title>Изменение экспедиции и дня</Card.Title>
              </Card.Header>
              <Card.Content>
                <div class="grid w-full items-center gap-4">
                  <div class="flex flex-col space-y-1.5">
                    <Label for="expedition">Экспедиция</Label>
                    <Select.Root bind:selected={expedition} name="expedition">
                      <Select.Trigger id="expedition">
                        <Select.Value placeholder="Выберите экспедицию" />
                      </Select.Trigger>
                      <Select.Content>
                        {#each expeditions as exp}
                          <Select.Item value={exp.value} label={exp.label}>
                            {exp.label}
                          </Select.Item>
                        {/each}
                      </Select.Content>
                    </Select.Root>
                  </div>
                  <div class="flex flex-col space-y-1.5">
                    <Label for="day">День</Label>
                    <Select.Root bind:selected={day} name="day">
                      <Select.Trigger id="day">
                        <Select.Value placeholder="Выберите день" />
                      </Select.Trigger>
                      <Select.Content class="max-h-[200px] overflow-y-auto">
                        {#each days as d}
                          <Select.Item value={d.value} label={d.label}>
                            {d.label}
                          </Select.Item>
                        {/each}
                      </Select.Content>
                    </Select.Root>
                  </div>
                </div>
              </Card.Content>
              <Card.Footer class="flex justify-between">
                <Button
                  variant="outline"
                  on:click={() => previousDay()}
                  disabled={disabledPreviousDay}>Предыдущий</Button
                >
                <Button
                  variant="outline"
                  on:click={() => nextDay()}
                  disabled={disabledNextDay}>Следующий</Button
                >
              </Card.Footer>
            </Card.Root>

            <Card.Root
              class="w-full bg-gradient-to-b from-white to-gray-50 shadow-md duration-300 rounded-xl"
            >
              <Card.Header>
                <Card.Title>Создание корабля</Card.Title>
              </Card.Header>
              <Card.Content>
                <form>
                  <div class="grid w-full items-center gap-4">
                    <div class="flex flex-col space-y-1.5">
                      <Label for="color">Цвет</Label>
                      <Select.Root
                        selected={selectedColor}
                        onSelectedChange={(selected) => {
                          if (selected) {
                            selectedColor = {
                              ...selectedColor,
                              value: selected.value,
                            };
                          }
                        }}
                      >
                        <Select.Trigger id="color">
                          <Select.Value placeholder="Выберите цвет" />
                        </Select.Trigger>
                        <Select.Content>
                          {#each colors as color}
                            <Select.Item
                              value={color.value}
                              label={color.label}
                            >
                              <div
                                class="w-4 h-4 rounded-md mr-2"
                                style="background-color: {color.hex}"
                              ></div>
                              {color.label}
                            </Select.Item>
                          {/each}
                        </Select.Content>
                      </Select.Root>
                    </div>
                    <div class="flex flex-col space-y-1.5">
                      <Label for="number">Номер</Label>
                      <Select.Root
                        selected={selectedNumber}
                        onSelectedChange={(selected) => {
                          if (selected) {
                            selectedNumber = {
                              ...selectedNumber,
                              value: selected.value,
                            };
                          }
                        }}
                      >
                        <Select.Trigger id="number">
                          <Select.Value placeholder="Выберите номер" />
                        </Select.Trigger>
                        <Select.Content>
                          {#each numbers as number}
                            <Select.Item
                              value={number.value}
                              label={number.label}
                            >
                              {number.label}
                            </Select.Item>
                          {/each}
                        </Select.Content>
                      </Select.Root>
                    </div>
                  </div>
                </form>
              </Card.Content>
              <Card.Footer class="flex flex-col gap-2 items-center">
                <Button on:click={addShip} class="w-full">Добавить</Button>

                <Button
                  on:click={() => addOtherObject("pirates")}
                  variant="outline"
                  class="w-full"
                >
                  Добавить пиратов
                </Button>
                <Button
                  on:click={() => addOtherObject("aborigines")}
                  variant="outline"
                  class="w-full"
                >
                  Добавить аборигенов
                </Button>
                <Button
                  on:click={deleteAllShips}
                  variant="outline"
                  class="text-red-500 hover:text-red-500 w-full"
                  ><Trash2 class="mr-2 h-4 w-4" />Удалить все</Button
                >
              </Card.Footer>
            </Card.Root>
          </div>
        </div>
      </Resizable.Pane>
    </Resizable.PaneGroup>
  </div>
</div>

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
    padding: 0 16px 20px;
    overflow-y: auto;
    max-height: calc(100vh - 100px);
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
    max-height: 100vh;
    overflow: hidden;
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
