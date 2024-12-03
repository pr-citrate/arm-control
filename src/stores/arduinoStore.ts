import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface ArduinoStatus {
  servo_angles: number[];
  digital_outputs: boolean[];
  digital_inputs: boolean[];
}

export interface DigitalInput {
  pin: number;
  state: boolean;
}

function createArduinoStore() {
  const { subscribe, set } = writable<ArduinoStatus | null>(null);
  let intervalId: number | null = null;

  async function updateStatus() {
    try {
      const arduinoStatus = await invoke<ArduinoStatus>("read_status");
      set(arduinoStatus);
    } catch (error) {
      console.error("상태 읽기 실패:", error);
    }
  }

  function startPolling() {
    if (!intervalId) {
      updateStatus(); // 즉시 첫 업데이트 실행
      intervalId = setInterval(updateStatus, 500) as unknown as number;
    }
  }

  function stopPolling() {
    if (intervalId) {
      clearInterval(intervalId);
      intervalId = null;
    }
  }

  return {
    subscribe,
    startPolling,
    stopPolling,
    updateStatus,
  };
}

export const arduinoStore = createArduinoStore(); 