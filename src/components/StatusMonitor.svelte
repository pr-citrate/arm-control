<script lang="ts">
    export let status: {
        servo_angles: number[];
        digital_outputs: boolean[];
        digital_inputs: boolean[];
    } | null = null;

    // 통신 로그를 저장할 배열
    let messageLog: {
        timestamp: string;
        direction: "tx" | "rx";
        message: string;
    }[] = [];
    const maxLogLength = 100; // 최대 로그 저장 개수

    // 최신 통신 데이터 업데이트 함수
    export function updateCommunication(command: string, response: string) {
        const timestamp = new Date().toLocaleTimeString();

        // 새 메시지 추가
        messageLog = [
            { timestamp, direction: "tx" as const, message: command },
            { timestamp, direction: "rx" as const, message: response },
            ...messageLog,
        ].slice(0, maxLogLength);

        // 콘솔에 통신 내용 출력
        console.log(`[${timestamp}] TX: ${command}`);
        console.log(`[${timestamp}] RX: ${response}`);
    }

    // 로그 지우기
    function clearLog() {
        messageLog = [];
    }
</script>

<div class="p-4">
    <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-semibold text-sky-700">시리얼 모니터</h2>
        <button
            on:click={clearLog}
            class="px-3 py-1 bg-sky-500 text-white text-sm rounded hover:bg-sky-600"
        >
            로그 지우기
        </button>
    </div>

    <div
        class="bg-gray-900 text-gray-100 p-4 rounded-lg h-[400px] overflow-y-auto font-mono text-sm"
    >
        {#each messageLog as log}
            <div class="mb-1">
                <span class="text-gray-500">[{log.timestamp}]</span>
                {#if log.direction === "tx"}
                    <span class="text-blue-400">TX ▶ </span>
                {:else}
                    <span class="text-green-400">RX ◀ </span>
                {/if}
                <span class="break-all">{log.message}</span>
            </div>
        {/each}
        {#if messageLog.length === 0}
            <div class="text-gray-500 italic">통신 기록이 없습니다...</div>
        {/if}
    </div>

    {#if status}
        <div
            class="mt-4 grid grid-cols-3 gap-2 text-xs bg-gray-100 p-2 rounded"
        >
            <div>
                <span class="text-gray-600">서보:</span>
                <span class="font-mono">{status.servo_angles.join(",")}</span>
            </div>
            <div>
                <span class="text-gray-600">출력:</span>
                <span class="font-mono">
                    {status.digital_outputs
                        .map((v) => (v ? "1" : "0"))
                        .join(",")}
                </span>
            </div>
            <div>
                <span class="text-gray-600">입력:</span>
                <span class="font-mono">
                    {status.digital_inputs
                        .map((v) => (v ? "1" : "0"))
                        .join(",")}
                </span>
            </div>
        </div>
    {/if}
</div>

<style>
    /* 스크롤바 스타일링 */
    div::-webkit-scrollbar {
        width: 8px;
    }

    div::-webkit-scrollbar-track {
        background: #374151;
        border-radius: 4px;
    }

    div::-webkit-scrollbar-thumb {
        background: #4b5563;
        border-radius: 4px;
    }

    div::-webkit-scrollbar-thumb:hover {
        background: #6b7280;
    }
</style>
