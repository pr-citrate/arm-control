<script>
    export let inputs = [
        { pin: 2, state: false },
        { pin: 4, state: false },
        { pin: 7, state: false },
    ];
</script>

<div class="p-4">
    <h2 class="text-lg font-semibold text-sky-700">Digital Inputs</h2>
    {#each inputs as input}
        <div class="flex items-center mt-4">
            <span class="w-16">Pin {input.pin}</span>
            <span
                class={`ml-auto px-4 py-2 rounded ${input.state ? "bg-green-500 text-white" : "bg-red-500 text-white"}`}
            >
                {input.state ? "HIGH" : "LOW"}
            </span>
        </div>
    {/each}
</div>
