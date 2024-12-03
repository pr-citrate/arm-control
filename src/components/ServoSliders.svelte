<script lang="ts">
    import { servoStore } from "../stores/servo";
    import { arduinoStore } from "../stores/arduinoStore";
    import { invoke } from "@tauri-apps/api/core";

    // arduinoStore의 상태가 변경될 때마다 서보 각도 업데이트
    $: if ($arduinoStore) {
        $arduinoStore.servo_angles.forEach((angle, index) => {
            servoStore.updateAngle(index + 1, angle);
        });
    }

    async function updateServos(id: number, angle: number) {
        // 현재 모든 서보 각도 가져오기
        const angles = $servoStore.map((servo) => servo.angle);

        // 디지털 출력 상태는 현재 상태 유지
        const digital_outputs = $arduinoStore?.digital_outputs || [
            false,
            false,
            false,
        ];

        try {
            await invoke("send_command", {
                command: {
                    angles,
                    digital_outputs,
                },
            });
            await arduinoStore.updateStatus(); // 상태 즉시 업데이트
        } catch (error) {
            console.error("서보 명령 전송 실패:", error);
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
