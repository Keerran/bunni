<script lang="ts">
	import { getConnectors } from "$lib/backend";
	import Icon from "./Icon.svelte";
    
    let connectors: string[] = [];

    getConnectors().then(c => connectors = c)

    $: console.log(connectors)
</script>
<div class="flex-1 flex text-gray-300 overflow-hidden">
    <div class="m-4 mr-0 w-60 flex flex-col">
        <div class="mb-2 rounded-md bg-main-light">
            <a href="/"
               class="w-full p-2 hover:bg-white hover:bg-opacity-5
                      text-center flex justify-between items-center">
                <Icon class="m-2 text-white-500">home</Icon>
                <div>Home</div>
                <div class="w-10"></div>
            </a>
            <a href="/liked"
               class="w-full p-2 hover:bg-white hover:bg-opacity-5
                      text-center flex justify-between items-center">
                <Icon class="m-2 text-white-500">favorite</Icon>
                <div>Library</div>
                <div class="w-10"></div>
            </a>
        </div>
        <div class="flex-1 rounded-md bg-main-light overflow-hidden">
            <ul>
                {#each connectors as connector, idx (idx)}
                    <li>
                        <a href={`/connector/${idx}`}
                           class="w-full p-2 hover:bg-white hover:bg-opacity-5 text-center inline-block">
                                {connector}
                        </a>
                    </li>
                {/each}
            </ul>
        </div>
    </div>
    <div class="m-4 flex-1 rounded-md bg-main-light relative flex overflow-hidden">
        <div class="flex-1 scrollbar overflow-auto p-4">
            <slot></slot>
        </div>
        <div class="absolute bottom-0 left-0 right-4 h-4 bg-gradient-to-t from-main-light"/>
        <div class="absolute top-0 left-0 right-4 h-4 bg-gradient-to-b from-main-light"/>
    </div>
</div>
