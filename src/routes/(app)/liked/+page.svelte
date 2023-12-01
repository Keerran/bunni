<script lang="ts">
    import MangaList from "$lib/components/MangaList.svelte";
    import WithSidebar from "$lib/components/WithSidebar.svelte";
    import * as commands from "$lib/backend";
    const liked = commands.fetchLiked().then(liked => liked.map(l => ({
        ...l[1].desc,
        connectorIdx: l[0]
    })));

</script>
<WithSidebar>
    {#await liked then liked}
        <MangaList mangas={liked} link={l => `/connector/${l.connectorIdx}/${l.id}`}/>
    {/await}
</WithSidebar>
