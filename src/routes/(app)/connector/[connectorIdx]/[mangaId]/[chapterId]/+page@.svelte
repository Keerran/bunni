<script lang="ts">
    import { page } from "$app/stores";
	import { fetchChapter } from "$lib/backend";
    import Carousel from "$lib/components/Carousel.svelte";
    import LongStrip from "$lib/components/LongStrip.svelte";
    import BackButton from "$lib/components/BackButton.svelte";
	import { appWindow } from "@tauri-apps/api/window";

    const {connectorIdx, chapterId} = $page.params;

    const chapter = fetchChapter(+connectorIdx, chapterId);
</script>
<div id="outer" class="relative bg-main-darker overflow-hidden h-screen w-screen flex">
    {#await chapter then chapter}
        {#if chapter.format === "Normal"}
            <Carousel {chapter} />
        {:else}
            <LongStrip {chapter} />
        {/if}
        <BackButton class="mt-4 mr-4" clickHandler={() => appWindow.close()}/>
    {/await}
    <div class="absolute w-full h-2" data-tauri-drag-region />
</div>
<style>
    #outer {
        filter: brightness(0.8) contrast(1.2)
    }
</style>
