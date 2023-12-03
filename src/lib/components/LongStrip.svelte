<script lang="ts">
	import type { ChapterImages } from "$lib/backend";
	import { createEventDispatcher } from "svelte";

    export let chapter: ChapterImages;
    export let width = "700px";

    const dispatch = createEventDispatcher();

    function onScroll(e: UIEvent) {
        const target = e.target as HTMLUListElement;
        if(target.scrollTop >= target.scrollHeight - target.clientHeight) {
            dispatch("finish")
        }
    }
</script>
<div class="flex-1 overflow-hidden flex">
        <ul class="flex-1 flex flex-col overflow-scroll scrollbar"
            on:scroll={onScroll}>
            {#each chapter.images as image}
                <li class="flex justify-center items-center">
                    <img alt="idk dude" style:width src={image}/>
                </li>
            {/each}
        </ul>
    </div>
