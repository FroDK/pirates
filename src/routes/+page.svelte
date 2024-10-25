<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";

  let ip = $state("127.0.0.1");
  let port = $state(9001);

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
</script>

<main>
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
