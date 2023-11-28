<script lang="ts">
    import Icon from "./Icon.svelte";

    export let query: string;
</script>
<form class="flex justify-start items-center w-full ring-indigo-500
             rounded-md bg-main-dark mb-4 focus-within:ring-2"
        on:submit|preventDefault>
    <button class="px-2" type="submit">
        <Icon>search</Icon>
    </button>
    <input class="flex-1 h-12 bg-transparent text-gray-300 focus:outline-none"
           type="text" name="query" on:change bind:value={query}/>
</form>