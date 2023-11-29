<script lang="ts">
    import { page } from "$app/stores";
	import { fetchChapter } from "$lib/backend";
    import Carousel from "$lib/components/Carousel.svelte";
    import LongStrip from "$lib/components/LongStrip.svelte";
    import BackButton from "$lib/components/BackButton.svelte";

    const {connectorIdx, chapterId} = $page.params;

    const chapter = fetchChapter(+connectorIdx, chapterId);
</script>
{#await chapter then chapter}
    {#if chapter.format === "Normal"}
        <Carousel {chapter} />
    {:else}
        <LongStrip {chapter} />
    {/if}
    <BackButton class="mt-4 mr-4"/>
{/await}
