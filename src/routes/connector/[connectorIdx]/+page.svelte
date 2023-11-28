<script lang="ts">
    import { searchManga, type SearchItem } from "$lib/backend";
	import { page } from "$app/stores";
	import MangaList from "$lib/components/MangaList.svelte";
    import SearchBar from "$lib/components/SearchBar.svelte";
    import WithSidebar from "$lib/components/WithSidebar.svelte";
    
    let connectorIdx = +$page.params.connectorIdx;
    let items = [] as SearchItem[];
    let query = "";

    async function search() {
        items = await searchManga(connectorIdx, query)
    }

    search()
</script>
<WithSidebar>
    <SearchBar bind:query on:submit={search}/>
    <MangaList mangas={items} link={(manga) => `/connector/${connectorIdx}/${manga.id}`}/>
</WithSidebar>
