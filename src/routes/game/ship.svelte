<script lang="ts">
  const {
    ship,
    colors,
    numbers,
    onRemove,
    onPositionChange,
    onSetAttackingShip,
    onSetDefendingShip,
    attackingShip,
    defendingShip,
    onColorChange,
    onNumberChange,
    onSizeChange,
  } = $props<{
    ship: any;
    colors: any[];
    numbers: any[];
    onRemove: (id: string) => void;
    onPositionChange: (id: string, x: number, y: number) => void;
    onSetAttackingShip: () => void;
    onSetDefendingShip: () => void;
    onColorChange: (color: string) => void;
    onNumberChange: (number: number) => void;
    onSizeChange: (width: number, height: number) => void;
    attackingShip: string | null | undefined;
    defendingShip: string | null | undefined;
  }>();

  import * as ContextMenu from "$lib/components/ui/context-menu";
  import { Trash2, PenLine, Sword, Shield } from "lucide-svelte";
  import { onMount } from "svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Slider } from "@/components/ui/slider";

  let shipElement: HTMLDivElement;

  onMount(() => {
    let isDragging = false;

    shipElement.addEventListener("mousedown", (e) => {
      if (e.button === 0) {
        isDragging = true;
        shipElement.style.cursor = "move";
        e.preventDefault();
      }
    });

    document.addEventListener("mousemove", (e) => {
      if (isDragging) {
        const mapRect = shipElement.parentElement!.getBoundingClientRect();
        let x = ((e.clientX - mapRect.left) / mapRect.width) * 100;
        let y = ((e.clientY - mapRect.top) / mapRect.height) * 100;

        // Ограничиваем значения x и y в пределах от 0 до 100
        x = Math.max(2, Math.min(98, x));
        y = Math.max(2, Math.min(94, y));

        shipElement.style.left = `${x}%`;
        shipElement.style.top = `${y}%`;
      }
    });

    document.addEventListener("mouseup", (e) => {
      if (isDragging) {
        isDragging = false;
        shipElement.style.cursor = "pointer";
        const mapRect = shipElement.parentElement!.getBoundingClientRect();
        let x = ((e.clientX - mapRect.left) / mapRect.width) * 100;
        let y = ((e.clientY - mapRect.top) / mapRect.height) * 100;

        // Ограничиваем значения x и y в пределах от 0 до 100
        x = Math.max(2, Math.min(98, x));
        y = Math.max(2, Math.min(94, y));

        onPositionChange(ship.id, x, y);
      }
    });
  });

  let dialogOpen = $state(false);
  let shipSize = $state([2]);

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
</script>

<div
  bind:this={shipElement}
  class="ship-on-map"
  style="background-image: url('/gifs/{ship.number}_{ship.color}.gif'); background-size: contain; left: {ship.x}%; top: {ship.y}%; width: {ship.width}px; height: {ship.height}px;"
>
  <ContextMenu.Root>
    <ContextMenu.Trigger class="context"></ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item
        disabled={ship.isAttacking ||
          attackingShip ||
          attackingShip === ship.id ||
          ship.isDefending}
        on:click={() => onSetAttackingShip(ship.id)}
      >
        <Sword class="mr-2 h-4 w-4" />
        <span>Атаковать</span>
      </ContextMenu.Item>
      <ContextMenu.Item
        disabled={ship.isDefending ||
          defendingShip ||
          defendingShip === ship.id ||
          ship.isAttacking}
        on:click={() => onSetDefendingShip(ship.id)}
      >
        <Shield class="mr-2 h-4 w-4" />
        <span>Обороняться</span>
      </ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item on:click={() => (dialogOpen = true)}>
        <PenLine class="mr-2 h-4 w-4" />
        <span>Редактировать</span>
      </ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item
        on:click={() => onRemove(ship.id)}
        class="text-red-500 data-[highlighted]:text-red-500"
      >
        <Trash2 class="mr-2 h-4 w-4" />
        <span>Удалить корабль</span></ContextMenu.Item
      >
    </ContextMenu.Content>
  </ContextMenu.Root>
</div>

<Dialog.Root bind:open={dialogOpen}>
  <Dialog.Trigger />
  <Dialog.Content class="sm:max-w-[425px]" overlay={false}>
    <Dialog.Header>
      <Dialog.Title>Редактирование корабля</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <!-- Редактирование размера корабля -->
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="name" class="text-right">Размер</Label>
        <Slider
          id="size"
          value={[
            Number(
              Object.entries(sizeMap).find(
                ([_, size]) =>
                  size.width === ship.width && size.height === ship.height
              )?.[0] ?? 2
            ),
          ]}
          onValueChange={(e) => {
            onSizeChange(sizeMap[e[0]].width, sizeMap[e[0]].height);
          }}
          max={4}
          step={1}
          class="col-span-3"
        />
      </div>

      <!-- Редактирование цвета корабля -->
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="username" class="text-right">Цвет</Label>
        <Select.Root
          portal={null}
          selected={ship.color}
          onSelectedChange={(selected) => {
            if (selected) {
              onColorChange(selected.value);
            }
          }}
        >
          <Select.Trigger class="w-[180px]">
            <Select.Value placeholder="Выберите цвет" />
          </Select.Trigger>
          <Select.Content>
            {#each colors as color}
              <Select.Item value={color.value}>
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

      <!-- Редактирование номера корабля -->
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="username" class="text-right">Номер</Label>
        <Select.Root
          portal={null}
          selected={ship.number}
          onSelectedChange={(selected) => {
            if (selected) {
              onNumberChange(selected.value);
            }
          }}
        >
          <Select.Trigger class="w-[180px]">
            <Select.Value placeholder="Выберите номер" />
          </Select.Trigger>
          <Select.Content>
            {#each numbers as number}
              <Select.Item value={number.value}>{number.label}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
  </Dialog.Content>
</Dialog.Root>

<style>
  :global(.ship-on-map) {
    position: absolute;
    transform: translate(-50%, -50%);
    cursor: pointer;
  }

  :global(.context) {
    height: 100%;
    width: 100%;
  }
</style>
