<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import PortSelector from "../components/PortSelector.svelte";
  import ServoSliders from "../components/ServoSliders.svelte";
  import DigitalOutputs from "../components/DigitalOutputs.svelte";
  import DigitalInputs from "../components/DigitalInputs.svelte";
  import { arduinoStore } from "../stores/arduinoStore";
  import type { DigitalInput } from "../stores/arduinoStore";

  let digitalInputs: DigitalInput[] = [];

  // arduinoStore 구독
  $: if ($arduinoStore) {
    digitalInputs = $arduinoStore.digital_inputs.map((state, index) => ({
      pin: [2, 4, 7][index],
      state,
    }));
  }

  onMount(() => {
    arduinoStore.startPolling();
  });

  onDestroy(() => {
    arduinoStore.stopPolling();
  });
</script>

<main class="min-h-screen bg-sky-50 p-8">
  <h1 class="text-2xl font-bold text-sky-800 mb-8">Tauri Arduino 컨트롤러</h1>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <PortSelector />
    <ServoSliders />
    <DigitalOutputs />
    <DigitalInputs inputs={digitalInputs} />
  </div>
</main>
