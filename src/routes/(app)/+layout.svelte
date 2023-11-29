<script lang="ts">
	import Icon from "$lib/components/Icon.svelte";
	import { appWindow } from "@tauri-apps/api/window";
</script>
<div class="h-screen bg-main flex flex-col overflow-hidden">
    <div
        id="titlebar"
        data-tauri-drag-region
        class="h-8 bg-main-dark select-none text-gray-300
            flex justify-end items-center"
    >
        <button on:click={() => appWindow.minimize()}>
            <Icon>minimize</Icon>
        </button>
        <button on:click={() => appWindow.toggleMaximize()}>
            <Icon>check_box_outline_blank</Icon>
        </button>
        <button on:click={() => appWindow.close()}>
            <Icon>close</Icon>
        </button>
    </div>
    <main class="bg-main flex-1 flex overflow-hidden relative">
        <slot />
    </main>
</div>

<style lang="postcss">
    #titlebar button {
        @apply px-2 h-full flex justify-center items-center hover:bg-white hover:bg-opacity-5;
        -webkit-app-region: no-drag;
    }
</style>
