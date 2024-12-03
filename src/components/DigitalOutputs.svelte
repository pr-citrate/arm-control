<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { servoStore } from "../stores/servo";
    import { arduinoStore } from "../stores/arduinoStore";

    interface Output {
        id: number;
        pin: number;
        state: boolean;
    }

    let outputs: Output[] = [
        { id: 1, pin: 8, state: false },
        { id: 2, pin: 12, state: false },
        { id: 3, pin: 13, state: false },
    ];

    // arduinoStore의 상태가 변경될 때마다 출력 상태 업데이트
    $: if ($arduinoStore) {
        outputs = outputs.map((output, index) => ({
            ...output,
            state: $arduinoStore.digital_outputs[index],
        }));
    }

    async function toggleOutput(output: Output) {
        const newState = !output.state;

        // 현재 서보 각도 상태 가져오기
        const angles = $servoStore.map((servo) => servo.angle);

        // 새로운 디지털 출력 상태 배열 생성
        const digital_outputs = outputs.map((o, index) =>
            index === output.id - 1 ? newState : o.state,
        );

        try {
            await invoke("send_command", {
                command: {
                    angles,
                    digital_outputs,
                },
            });
            await arduinoStore.updateStatus(); // 상태 즉시 업데이트
        } catch (error) {
            console.error("디지털 출력 전환 실패:", error);
        }
    }
</script>

<div class="p-4">
    <h2 class="text-lg font-semibold text-sky-700">디지털 출력</h2>
    {#each outputs as output}
        <div class="mt-4 flex items-center justify-between">
            <span class="text-sm text-sky-600">Pin {output.pin}</span>
            <button
                on:click={() => toggleOutput(output)}
                class={`px-4 py-2 rounded ${
                    output.state
                        ? "bg-green-500 hover:bg-green-600"
                        : "bg-red-500 hover:bg-red-600"
                } text-white transition-colors`}
            >
                {output.state ? "HIGH" : "LOW"}
            </button>
        </div>
    {/each}
</div>
