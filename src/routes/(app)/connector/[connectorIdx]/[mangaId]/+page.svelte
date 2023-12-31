<script lang="ts">
    import { page } from "$app/stores";
    import BackButton from "$lib/components/BackButton.svelte";
    import Icon from "$lib/components/Icon.svelte";
    import WithSidebar from "$lib/components/WithSidebar.svelte";
    import { fetchManga, isLiked, toggleLiked, type Chapter} from "$lib/backend";
	import { WebviewWindow } from "@tauri-apps/api/window";
    import { type Event, listen } from "@tauri-apps/api/event";

    let {connectorIdx, mangaId} = $page.params;

    let manga = fetchManga(+connectorIdx, mangaId);
    let liked = false;
    isLiked(+connectorIdx, mangaId).then(val => liked = val);

    async function like() {
        liked = await toggleLiked(+connectorIdx, mangaId);
    }

    interface ReadEvent {
        connector_idx: number,
        chapter_id: string,
    }

    listen("chapter_read", async ({ payload }: Event<ReadEvent>) => {
        console.log(payload);
        if(payload.connector_idx === +connectorIdx) {
            const mangaData = await manga
            const chapter = mangaData.chapters.find((c: Chapter) => c.id === payload.chapter_id);
            if (chapter) {
                chapter.read = true
                manga = Promise.resolve(mangaData)
            }
        }
    })

    async function openChapter(chapterId: string) {
        const href = `/connector/${connectorIdx}/${mangaId}/${chapterId}`;
        // TODO give proper window label
        new WebviewWindow("chapter", {
            url: href,
            decorations: false,
            maximized: true,
        })
    }
</script>
<WithSidebar>
    <div class="relative">
        {#await manga then manga}
            <div>
                <div class="w-full flex" style:height="23rem">
                    <div class="bg-main rounded-md">
                        <img alt={manga.desc.title} class="h-80 rounded-t-md" src={manga.desc.cover_url}/>
                        <div class="w-full flex rounded-b-md overflow-hidden">
                            <button on:click={like}
                                    class="h-12 transparent-button">
                                <Icon class={liked ? "liked" : ""}>favorite</Icon>
                            </button>
                            <button class="h-12 transparent-button">
                                <Icon>track_changes</Icon>
                            </button>
                            <!-- <TrackerModal manga={manga} connectorIdx={+idx} anilistId={anilistId}
                                            onClose={() => setModalOpen(false)} open={modalOpen}/> -->
                        </div>
                    </div>
                    <div class="flex-1 flex flex-col h-full px-4">
                        <div class="font-semibold text-4xl">
                            {manga.desc.title}
                        </div>
                        <!-- <EllipsisText class="flex-1 mt-2 leading-normal">
                            {manga.desc.description}
                        </EllipsisText> -->
                    </div>
                </div>
                <ul class="w-full bg-main mt-4 rounded-md shadow-inner overflow-hidden">
                    {#each manga.chapters as chapter (chapter.id)}
                        <li class="flex group">
                            <button on:click={() => openChapter(chapter.id)}
                               class="w-full px-4 py-2 group-first:pt-3 group-last:pb-3
                                    hover:bg-main-dark transition-colors duration-150"
                               class:bg-main-darker={chapter.read}>
                                {chapter.name}
                            </button>
                        </li>
                    {/each}
                </ul>
            </div>
        {/await}
    <BackButton/>
    </div>
</WithSidebar>

<style lang="postcss">
    .transparent-button {
        @apply bg-black bg-opacity-0 hover:bg-opacity-20 active:bg-opacity-40
        flex-1 p-2 transition-colors duration-150;
    }
</style>
