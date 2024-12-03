<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { servoStore } from "../stores/servo";

    interface Output {
        id: number;
        pin: number;
        state: boolean;
    }

    let outputs = [
        { id: 1, pin: 8, state: false },
        { id: 2, pin: 12, state: false },
        { id: 3, pin: 13, state: false },
    ];

    async function toggleOutput(output: Output) {
        output.state = !output.state;

        // 현재 서보 각도 상태 가져오기
        const angles = $servoStore.map((servo) => servo.angle);

        // 현재 디지털 출력 상태 배열 생성
        const digital_outputs = outputs.map((o) => o.state);

        try {
            await invoke("send_command", {
                command: {
                    angles,
                    digital_outputs,
                },
            });
        } catch (error) {
            console.error("Failed to send digital output command:", error);
            // 실패시 상태 되돌리기
            output.state = !output.state;
        }
    }
</script>

<div class="p-4">
    <h2 class="text-lg font-semibold text-sky-700">디지털 출력</h2>
    {#each outputs as output}
        <div class="flex items-center mt-4">
            <span class="w-16">핀 {output.pin}</span>
            <button
                on:click={() => toggleOutput(output)}
                class="ml-auto px-4 py-2 bg-sky-500 text-white rounded hover:bg-sky-600"
            >
                {output.state ? "켜짐" : "꺼짐"}
            </button>
        </div>
    {/each}
</div>
