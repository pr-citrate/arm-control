<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import PortSelector from "../components/PortSelector.svelte";
  import ServoSliders from "../components/ServoSliders.svelte";
  import DigitalOutputs from "../components/DigitalOutputs.svelte";
  import DigitalInputs from "../components/DigitalInputs.svelte";
  import StatusMonitor from "../components/StatusMonitor.svelte";

  interface ArduinoStatus {
    servo_angles: number[];
    digital_outputs: boolean[];
    digital_inputs: boolean[];
  }

  interface DigitalInput {
    pin: number;
    state: boolean;
  }

  let status: ArduinoStatus | null = null;
  let digitalInputs: DigitalInput[] = [];
  let updateInterval: number;
  let statusMonitorComponent: InstanceType<typeof StatusMonitor>;

  async function updateStatus() {
    try {
      const arduinoStatus = await invoke<ArduinoStatus>("read_status");
      status = arduinoStatus;

      digitalInputs = arduinoStatus.digital_inputs.map((state, index) => ({
        pin: [2, 4, 7][index],
        state,
      }));

      const rawData =
        `S${arduinoStatus.servo_angles.join(",")}` +
        `,${arduinoStatus.digital_outputs.map((s) => (s ? "1" : "0")).join(",")}` +
        `,${arduinoStatus.digital_inputs.map((s) => (s ? "1" : "0")).join(",")}E`;
      statusMonitorComponent?.updateCommunication("READ_STATUS", rawData);
    } catch (error) {
      console.error("Failed to read status:", error);
    }
  }

  onMount(() => {
    updateInterval = setInterval(updateStatus, 500);
  });

  onDestroy(() => {
    clearInterval(updateInterval);
  });
</script>

<main class="min-h-screen bg-sky-50 p-8">
  <h1 class="text-2xl font-bold text-sky-800 mb-8">Tauri Arduino 컨트롤러</h1>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <PortSelector />
    <ServoSliders />
    <DigitalOutputs />
    <DigitalInputs inputs={digitalInputs} />
    <StatusMonitor bind:this={statusMonitorComponent} {status} />
  </div>
</main>
