<script>
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    let outputs = [
        { id: 1, pin: 8, state: false },
        { id: 2, pin: 12, state: false },
        { id: 3, pin: 13, state: false },
    ];

    function toggleOutput(output) {
        output.state = !output.state;
        dispatch("toggleOutput", { pin: output.pin, state: output.state });
    }
</script>

<div class="p-4">
    <h2 class="text-lg font-semibold text-sky-700">Digital Outputs</h2>
    {#each outputs as output}
        <div class="flex items-center mt-4">
            <span class="w-16">Pin {output.pin}</span>
            <button
                on:click={() => toggleOutput(output)}
                class="ml-auto px-4 py-2 bg-sky-500 text-white rounded hover:bg-sky-600"
            >
                {output.state ? "ON" : "OFF"}
            </button>
        </div>
    {/each}
</div>
