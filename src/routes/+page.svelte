<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { check } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { Toaster } from "$lib/components/ui/sonner";
  import { toast } from "svelte-sonner";
  import { onMount } from "svelte";
  import type { Update } from "@tauri-apps/plugin-updater";
  import { Progress } from "$lib/components/ui/progress/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { getVersion } from "@tauri-apps/api/app";
  import { Command } from '@tauri-apps/plugin-shell';

  let ip = $state("127.0.0.1");
  let port = $state(9001);

  let update = $state<Update | null>(null);
  let isUpdateAvailable = $state(false);
  let showUpdateModal = $state(false);

  interface IDownloadState {
    downloadState: "Started" | "Progress" | "Finished" | null;
    isDownloading: boolean;
    contentLength: number;
    downloaded: number;
  }

  let downloadState = $state<IDownloadState>({
    downloadState: null,
    isDownloading: false,
    contentLength: 0,
    downloaded: 0,
  });

  function connectToServer() {
    if (ip.trim() !== "" && port !== 0) {
      window.location.href = `/client?ip=${encodeURIComponent(ip)}&port=${encodeURIComponent(port)}`;
    } else {
      alert("Пожалуйста, введите IP-адрес и порт");
    }
  }

  function createGame() {
    window.location.href = `/game`;
  }

  async function downloadUpdate() {
    if (update) {
      downloadState.isDownloading = true;

      await update.download((event) => {
        switch (event.event) {
          case "Started":
            downloadState.downloadState = "Started";
            downloadState.contentLength = event.data.contentLength!;
            break;
          case "Progress":
            downloadState.downloadState = "Progress";
            downloadState.downloaded += event.data.chunkLength;
            break;
          case "Finished":
            downloadState.downloadState = "Finished";
            downloadState.isDownloading = false;
            break;
        }
      });
    }
  }

  async function installAndRestart() {
    if (update) {
      await update.install();
      await relaunch();
    }
  }

  async function getLocalIP() {
    try {
      const result = await Command.create('get-ip', ["(Get-NetIPAddress -AddressFamily IPv4 -InterfaceAlias 'Ethernet*').IPAddress"]).execute();
      
      if (result.stdout) {
        return result.stdout.trim();
      }
    } catch (error) {
      console.error('Ошибка при получении IP-адреса:', error);
    }
  }

  onMount(async () => {
    toast.promise(check(), {
      loading: "Проверка обновлений...",
      success: (_update: Update | null) => {
        isUpdateAvailable = _update !== null;
        update = _update;
        return isUpdateAvailable ? "Доступно обновление" : "Обновлений нет";
      },
      error: "Произошла ошибка при проверке обновлений",
    });
  });

  $effect(() => {
    if (isUpdateAvailable && update) {
      toast.success(`Обнаружена новая версия: ${update.version}`, {
        action: {
          label: "Подробнее",
          onClick: () => {
            showUpdateModal = true;
          },
        },
      });
    }
  });

  function formatDate(dateString: string): string {
    // Разбиваем строку на части
    const [datePart, timePart] = dateString.split(' ');
    const [year, month, day] = datePart.split('-');
    const [time] = timePart.split('.');
    const [hour, minute] = time.split(':');
    
    // Создаем объект Date
    const date = new Date(Number(year), Number(month) - 1, Number(day), Number(hour), Number(minute));
    
    // Форматируем дату
    return date.toLocaleString('ru-RU', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
</script>

<main>
  <Toaster position="top-right" theme="light" />

  {#if update}
    <Dialog.Root bind:open={showUpdateModal}>
      <Dialog.Content >
        <Dialog.Header>
          <Dialog.Title
            >Обновление: {update.version} от {formatDate(update.date!)}</Dialog.Title
          >
          <Dialog.Description>
            {update.body}
          </Dialog.Description>
        </Dialog.Header>
        <div>
          {#if downloadState.downloaded > 0}
            <Label>
              Загружено: {(downloadState.downloaded / 1024 / 1024).toFixed(2)}
              МБ из {(downloadState.contentLength / 1024 / 1024).toFixed(2)}
              МБ
            </Label>
            <Progress
              value={(downloadState.downloaded / downloadState.contentLength) *
                100 || parseInt("0")}
              max={100}
              class={downloadState.downloadState === "Finished" ? "progress-finished" : ""}
            />
            <Label>
              {(
                (downloadState.downloaded / downloadState.contentLength) *
                  100 || parseFloat("0.0")
              ).toFixed(1)}%
            </Label>
          {/if}
        </div>
        <Dialog.Footer class="flex justify-between">
          <Button variant="outline" onclick={() => (showUpdateModal = false)}
            >Отмена</Button
          >
          {#if !downloadState.isDownloading && downloadState.downloaded === 0}
            <Button onclick={downloadUpdate}>Загрузить</Button>
          {:else if downloadState.downloaded > 0 && downloadState.downloadState === "Progress"}
            <Button disabled>Загрузка...</Button>
          {:else if downloadState.downloadState === "Finished"}
            <Button onclick={installAndRestart}>Перезапустить</Button>
          {/if}
        </Dialog.Footer>
      </Dialog.Content>
    </Dialog.Root>
  {/if}

  <Card.Root class="w-[450px]">
    <Card.Content>
      <form>
        <div class="grid w-full items-center gap-4">
          <div class="flex flex-col space-y-1.5">
            <Label for="ip">IP</Label>
            <Input id="ip" bind:value={ip} />
          </div>
          <div class="flex flex-col space-y-1.5">
            <Label for="port">Port</Label>
            <Input id="port" bind:value={port} />
          </div>
        </div>
      </form>
    </Card.Content>
    <Card.Footer class="flex justify-between">
      <Button variant="outline" onclick={connectToServer}>
        Подключиться к серверу
      </Button>

      <Button onclick={createGame}>Создать игру</Button>
    </Card.Footer>
  </Card.Root>

  <div class="absolute left-0 bottom-0 w-full text-gray-500 font-mono text-xs flex justify-between">
    {#await getVersion()}
      <span>Загрузка...</span>
    {:then version}
      <span>v{version}</span>
    {/await}

    {#await getLocalIP()}
      <span>Загрузка...</span>
    {:then ip}
      <span>{ip}</span>
    {/await}
  </div>
</main>

<style>


  main {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
    width: 100%;
  }

  :global(.progress-finished :first-child) {
    background-color: #37ff37;
  }
  
  :global(body) {
    background-color: #f0f0f0;
  }
</style>
