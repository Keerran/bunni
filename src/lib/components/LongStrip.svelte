<script lang="ts">
    import type { ChapterImages } from "./backend";

    export let chapter: ChapterImages;
    export let width = "700px";
</script>
<div class="flex-1 overflow-hidden flex">
        <ul class="flex-1 flex flex-col overflow-scroll scrollbar">
            {#each chapter.images as image}
                <li class="flex justify-center items-center">
                    <img alt="idk dude" style:width src={image}/>
                </li>
            {/each}
        </ul>
    </div>