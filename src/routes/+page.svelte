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
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import { Progress } from "$lib/components/ui/progress/index.js";

  let ip = $state("127.0.0.1");
  let port = $state(9001);

  let update = $state<Update | null>(null);
  let isUpdateAvailable = $state(false);
  let showUpdateModal = $state(false);

  let downloadState = $state({
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

      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            downloadState.contentLength = event.data.contentLength!;
            break;
          case "Progress":
            downloadState.downloaded += event.data.chunkLength;
            break;
          case "Finished":
            downloadState.isDownloading = false;
            break;
        }
      });
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
      toast.success(`Обновление: ${update.version} от ${update.date}`, {
        action: {
          label: "Подробнее",
          onClick: () => {
            showUpdateModal = true;
          },
        },
      });
    }
  });
</script>

<main>
  <Toaster position="top-right" theme="light" />

  {#if update}
    <AlertDialog.Root bind:open={showUpdateModal}>
      <AlertDialog.Content>
        <AlertDialog.Header>
          <AlertDialog.Title>
            Обновление: {update.version} от {update.date}
          </AlertDialog.Title>
          <AlertDialog.Description>{update.body}</AlertDialog.Description>
          {#if downloadState.isDownloading}
            <AlertDialog.Description>
              <div>
                <Label>
                  Загружено: {(downloadState.downloaded / 1024 / 1024).toFixed(
                    2
                  )}
                  МБ из {(downloadState.contentLength / 1024 / 1024).toFixed(2)}
                  МБ
                </Label>
                <Progress
                  value={(downloadState.downloaded /
                    downloadState.contentLength) *
                    100}
                  max={100}
                />
                <Label>
                  {(
                    (downloadState.downloaded / downloadState.contentLength) *
                    100
                  ).toFixed(1)}%
                </Label>
              </div>
            </AlertDialog.Description>
          {/if}
        </AlertDialog.Header>
        <AlertDialog.Footer>
          <AlertDialog.Cancel>Отмена</AlertDialog.Cancel>
          {#if !downloadState.isDownloading}
            <AlertDialog.Action onclick={downloadUpdate}>
              Установить
            </AlertDialog.Action>
          {:else if downloadState.downloaded > 0}
            <AlertDialog.Action disabled>Установка...</AlertDialog.Action>
          {:else if downloadState.downloaded === downloadState.contentLength}
            <AlertDialog.Action onclick={relaunch}>
              Перезапустить
            </AlertDialog.Action>
          {/if}
        </AlertDialog.Footer>
      </AlertDialog.Content>
    </AlertDialog.Root>
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

  :global(body) {
    background-color: #f0f0f0;
  }
</style>
