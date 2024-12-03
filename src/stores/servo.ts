import { writable } from 'svelte/store';

export interface Servo {
  id: number;
  angle: number;
}

function createServoStore() {
  const { subscribe, update } = writable<Servo[]>([
    { id: 1, angle: 90 },
    { id: 2, angle: 90 },
    { id: 3, angle: 90 },
    { id: 4, angle: 90 },
    { id: 5, angle: 90 },
    { id: 6, angle: 90 },
  ]);

  return {
    subscribe,
    updateAngle: (id: number, angle: number) => {
      update(servos =>
        servos.map(servo =>
          servo.id === id ? { ...servo, angle } : servo
        )
      );
    }
  };
}

export const servoStore = createServoStore(); 