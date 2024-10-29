<script lang="ts">
  import { onMount } from "svelte";
  import * as ContextMenu from "$lib/components/ui/context-menu";
  import { Trash2 } from "lucide-svelte";

  interface IOtherObjects {
    id: string;
    x: number;
    y: number;
    width: number;
    height: number;
    path: string;
    onPositionChange: (id: string, x: number, y: number) => void;
    onRemove: (id: string) => void;
  }

  let {
    id,
    x,
    y,
    width,
    height,
    path,
    onPositionChange,
    onRemove,
  }: IOtherObjects = $props();

  let otherObjectElement: HTMLDivElement;

  onMount(() => {
    let isDragging = false;

    otherObjectElement.addEventListener("mousedown", (e) => {
      if (e.button === 0) {
        isDragging = true;
        otherObjectElement.style.cursor = "move";
        e.preventDefault();
      }
    });

    document.addEventListener("mousemove", (e) => {
      if (isDragging) {
        const mapRect =
          otherObjectElement.parentElement!.getBoundingClientRect();
        let x = ((e.clientX - mapRect.left) / mapRect.width) * 100;
        let y = ((e.clientY - mapRect.top) / mapRect.height) * 100;

        // Ограничиваем значения x и y в пределах от 0 до 100
        x = Math.max(2, Math.min(98, x));
        y = Math.max(2, Math.min(94, y));

        otherObjectElement.style.left = `${x}%`;
        otherObjectElement.style.top = `${y}%`;
      }
    });

    document.addEventListener("mouseup", (e) => {
      if (isDragging) {
        isDragging = false;
        otherObjectElement.style.cursor = "pointer";
        const mapRect =
          otherObjectElement.parentElement!.getBoundingClientRect();
        let x = ((e.clientX - mapRect.left) / mapRect.width) * 100;
        let y = ((e.clientY - mapRect.top) / mapRect.height) * 100;

        // Ограничиваем значения x и y в пределах от 0 до 100
        x = Math.max(2, Math.min(98, x));
        y = Math.max(2, Math.min(94, y));

        onPositionChange(id, x, y);
      }
    });
  });
</script>

<div
  bind:this={otherObjectElement}
  class="other-object"
  style="background-image: url('{path}'); background-size: contain; left: {x}%; top: {y}%; width: {width}px; height: {height}px;"
>
  <ContextMenu.Root>
    <ContextMenu.Trigger class="context"></ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item
        on:click={() => onRemove(id)}
        class="text-red-500 data-[highlighted]:text-red-500"
      >
        <Trash2 class="mr-2 h-4 w-4" />
        <span>Удалить</span></ContextMenu.Item
      >
    </ContextMenu.Content>
  </ContextMenu.Root>
</div>

<style>
  :global(.other-object) {
    position: absolute;
    transform: translate(-50%, -50%);
  }
</style>
