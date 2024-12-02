<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let ports: string[] = [];
  let selectedPort: string = "";

  async function fetchPorts(): Promise<void> {
    ports = await invoke<string[]>("list_serial_ports");
  }

  function connect(): void {
    invoke("connect_port", { port: selectedPort });
  }

  onMount(() => {
    fetchPorts();
  });
</script>

<div class="p-4">
  <label class="block text-sm font-medium text-sky-700" for="port_select"
    >Select Serial Port:</label
  >
  <select
    id="port_select"
    bind:value={selectedPort}
    class="mt-1 block w-full pl-3 pr-10 py-2 text-base border border-sky-300 focus:outline-none focus:ring-sky-500 focus:border-sky-500 sm:text-sm rounded-md"
  >
    {#each ports as port}
      <option value={port}>{port}</option>
    {/each}
  </select>
  <button
    on:click={connect}
    class="mt-4 px-4 py-2 bg-sky-500 text-white rounded hover:bg-sky-600"
  >
    Connect
  </button>
</div>
