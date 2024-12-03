<script lang="ts">
    import { servoStore } from "../stores/servo";
    import { invoke } from "@tauri-apps/api/core";

    async function updateServos(id: number, angle: number) {
        // 현재 모든 서보 각도 가져오기
        const angles = $servoStore.map((servo) => servo.angle);
        // 새로운 각도로 업데이트
        angles[id - 1] = angle;

        // 디지털 출력 상태는 변경하지 않음 (현재 상태 유지)
        const digital_outputs = [false, false, false]; // 기본값 또는 store에서 가져오기

        try {
            await invoke("send_command", {
                command: {
                    angles,
                    digital_outputs,
                },
            });
        } catch (error) {
            console.error("Failed to send servo command:", error);
        }
    }
</script>

<div class="p-4">
    <h2 class="text-lg font-semibold text-sky-700">서보 모터</h2>
    {#each $servoStore as servo}
        <div class="mt-4">
            <label class="block text-sm text-sky-600" for={`servo_${servo.id}`}
                >서보 {servo.id}: {servo.angle}°</label
            >
            <input
                id={`servo_${servo.id}`}
                type="range"
                min="0"
                max="180"
                bind:value={servo.angle}
                on:input={() => updateServos(servo.id, servo.angle)}
                class="w-full"
            />
        </div>
    {/each}
</div>
