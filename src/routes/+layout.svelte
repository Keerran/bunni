<script lang="ts">
    import Icon from "$lib/components/Icon.svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import "../app.css";
</script>

<svelte:head>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" />
    <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
    />
    <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700&display=swap"
    />
</svelte:head>
<slot />
<style lang="postcss">
    :global(.material-symbols-outlined) {
        font-variation-settings: "FILL" 0, "wght" 400, "GRAD" 0, "opsz" 48;
    }
</style>
