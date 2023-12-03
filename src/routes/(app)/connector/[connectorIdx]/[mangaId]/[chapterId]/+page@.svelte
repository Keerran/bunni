<script lang="ts">
    import { page } from "$app/stores";
	import { fetchChapter, getMangaView, setMangaView, markChapterRead } from "$lib/backend";
    import Carousel from "$lib/components/Carousel.svelte";
    import LongStrip from "$lib/components/LongStrip.svelte";
    import BackButton from "$lib/components/BackButton.svelte";
	import { appWindow } from "@tauri-apps/api/window";
	import Icon from "$lib/components/Icon.svelte";

    const {connectorIdx, mangaId, chapterId} = $page.params;

    const chapter = fetchChapter(+connectorIdx, chapterId);

    let isLong = false;

    chapter.then(async chapter => {
        const pref = await getMangaView(+connectorIdx, mangaId);

        isLong = pref === undefined ? chapter.format === "Long" : pref === "Long";
    })

    function changeType() {
        isLong = !isLong;
        setMangaView(+connectorIdx, mangaId, isLong);
    }

    function finishReading() {
        markChapterRead(+connectorIdx, chapterId);
    }

</script>
<div id="outer" class="relative bg-main-darker overflow-hidden h-screen w-screen flex">
    {#await chapter then chapter}
        {#if isLong}
            <LongStrip {chapter} on:finish={finishReading} />
        {:else}
            <Carousel {chapter} on:finish={finishReading} />
        {/if}
    {/await}
        <BackButton class="mt-4 mr-4" clickHandler={() => appWindow.close()}/>
    <button class="absolute top-12 right-4 transition-colors duration-150
                   hover:bg-white hover:bg-opacity-5 p-1 rounded-full text-white"
            class:text-indigo-500={isLong}
            on:click={changeType}>
        <Icon>view_day</Icon>
    </button>
    <div class="absolute w-full h-2" data-tauri-drag-region />
</div>
<style>
    #outer {
        filter: brightness(0.8) contrast(1.2)
    }
</style>
