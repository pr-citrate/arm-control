<script lang="ts">
  import PortSelector from "../components/PortSelector.svelte";
  import ServoSliders from "../components/ServoSliders.svelte";
  import DigitalOutputs from "../components/DigitalOutputs.svelte";
  import DigitalInputs from "../components/DigitalInputs.svelte";
  import StatusMonitor from "../components/StatusMonitor.svelte";

  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  interface DigitalInput {
    pin: number;
    state: boolean;
  }

  interface ServoEvent {
    detail: {
      id: number;
      angle: number;
    };
  }

  interface OutputEvent {
    detail: {
      pin: number;
      state: boolean;
    };
  }

  interface StatusEvent {
    payload: {
      digitalInputs?: DigitalInput[];
      [key: string]: unknown;
    };
  }

  let digitalInputs: DigitalInput[] = [
    { pin: 2, state: false },
    { pin: 4, state: false },
    { pin: 7, state: false },
  ];

  let status = "";

  // Handle servo updates
  function handleServoUpdate(event: ServoEvent) {
    invoke("send_servo_command", {
      id: event.detail.id,
      angle: event.detail.angle,
    });
  }

  // Handle digital output toggles
  function handleToggleOutput(event: OutputEvent) {
    invoke("send_digital_output", {
      pin: event.detail.pin,
      state: event.detail.state,
    });
  }

  // Listen for status updates from backend
  listen("status_update", (event: StatusEvent) => {
    const data = event.payload;
    status = String(data);
    // Update digital inputs if included in the status
    if (data.digitalInputs) {
      digitalInputs = data.digitalInputs;
    }
  });
</script>

<main class="min-h-screen bg-sky-50 p-8">
  <h1 class="text-2xl font-bold text-sky-800 mb-8">Tauri Arduino Controller</h1>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <PortSelector />
    <ServoSliders on:updateServo={handleServoUpdate} />
    <DigitalOutputs on:toggleOutput={handleToggleOutput} />
    <DigitalInputs inputs={digitalInputs} />
    <StatusMonitor {status} />
  </div>
</main>
