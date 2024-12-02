<script lang="ts">
    import { createEventDispatcher } from "svelte";

    interface ServoEvent {
        id: number;
        angle: number;
    }

    const dispatch = createEventDispatcher<{
        updateServo: ServoEvent;
    }>();

    interface Servo {
        id: number;
        angle: number;
    }

    let servos: Servo[] = [
        { id: 1, angle: 90 },
        { id: 2, angle: 90 },
        { id: 3, angle: 90 },
        { id: 4, angle: 90 },
        { id: 5, angle: 90 },
        { id: 6, angle: 90 },
    ];

    function updateAngle(servo: Servo, event: Event) {
        const target = event.target as HTMLInputElement;
        servo.angle = parseInt(target.value);
        dispatch("updateServo", { id: servo.id, angle: servo.angle });
    }
</script>

<div class="p-4">
    <h2 class="text-lg font-semibold text-sky-700">Servo Motors</h2>
    {#each servos as servo}
        <div class="mt-4">
            <label class="block text-sm text-sky-600" for={`servo_${servo.id}`}
                >Servo {servo.id}: {servo.angle}°</label
            >
            <input
                id={`servo_${servo.id}`}
                type="range"
                min="0"
                max="180"
                bind:value={servo.angle}
                on:input={(e) => updateAngle(servo, e)}
                class="w-full"
            />
        </div>
    {/each}
</div>
