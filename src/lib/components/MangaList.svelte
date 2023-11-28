<script lang="ts" generics="X extends SearchItem">
    // TODO: REMOVE WHEN https://github.com/sveltejs/svelte-eslint-parser/issues/306 IS FIXED
    // eslint-disable-next-line no-undef, @typescript-eslint/no-explicit-any
    type T = X | any
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    import type { SearchItem } from "$lib/backend";
    import { popIn } from "$lib/util";

    export let mangas: T[];
    export let link: (manga: T) => string;

    const gradient = "linear-gradient(to top, hsla(0, 0%, 0%, 0.8), transparent)";
</script>

<ul class="responsive-grid gap-4">
    {#each mangas as manga, i (manga.id)}
        <li class="relative rounded-md overflow-hidden"
            in:popIn={{delay: i * 30}}>
            <a href={link(manga)} class="contents">
                <div
                    class="relative w-full overflow-hidden"
                    style:padding-top="133.33%"
                >
                    <img
                        alt={manga.id}
                        class="absolute top-0 w-full"
                        src={manga.cover_url}
                    />
                </div>
                <div
                    class="absolute bottom-0 left-0 right-0 h-1/3"
                    style:background-image={gradient}
                />
                <div
                    class="absolute bottom-0 left-0 right-0
                           text-center text-white p-2"
                >
                    {manga.title}
                </div>
            </a>
        </li>
    {/each}
</ul>
<style lang="postcss">
    .responsive-grid {
        display: grid;
        align-items: center;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    }

    .responsive-grid > * {
        max-width: 320px;
    }

</style>
