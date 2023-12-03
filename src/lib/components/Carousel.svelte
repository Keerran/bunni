<script lang="ts">
    import { createEventDispatcher, onDestroy, onMount } from "svelte";
    // @ts-expect-error svelte-zoom has no typings
    import Zoom from "svelte-zoom";
    import type { CoordDiff, Coords, SwipeEvent } from "$lib/types";
    import { swipe } from "$lib/util";
	import type { ChapterImages } from "$lib/backend";

    export let chapter: ChapterImages;

    const dispatch = createEventDispatcher();

    let duration = 250;
    let index = 0;
    let offset = 0;
    let availableSpace: number;
    let swiping = false;

    onMount(() => {
        document.addEventListener("keydown", handleKeys);
    });

    onDestroy(() => {
        document.removeEventListener("keydown", handleKeys);
    });

    $: if(index === chapter.images.length) {
        dispatch("finish")
    }

    function handleKeys(e: KeyboardEvent) {
        switch(e.key) {
            case "ArrowRight":
                index--;
                break;
            case "ArrowLeft":
                index++;
                break;
        }
    }

    function handleStart(_e: CustomEvent<Coords>) {
        swiping = true;
    }

    function handleMove(e: CustomEvent<Coords & CoordDiff>) {
        offset += e.detail.dx;
    }

    function handleEnd(e: CustomEvent<SwipeEvent>) {
        offset = 0;
        swiping = false;
        index += e.detail.direction;
    }
</script>
<ul use:swipe={{ thresholdProvider: () => availableSpace }}
    on:swipeStart={handleStart}
    on:swipeMove={handleMove}
    on:swipeEnd={handleEnd}
    style:transform="translateX({index * availableSpace + offset}px)"
    style:transition-duration="{swiping ? 0 : duration}ms"
    bind:offsetWidth={availableSpace}
    class="transition-transform ease-in-out w-full flex flex-row-reverse justify-start items-stretch"
>
    {#each chapter.images as image (image)}
        <li style:flex="1 0 100%" class="flex justify-center items-stretch">
            <Zoom src={image} alt="idk man" />
        </li>
    {/each}
</ul>
