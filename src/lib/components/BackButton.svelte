<script lang="ts">
    import Icon from "./Icon.svelte";
</script>
<button class="absolute -top-2 -right-0 text-gray-500 transition-all
               duration-150 hover:bg-white hover:bg-opacity-5 p-1 rounded-full
               {$$restProps.class}"
        on:click={() => history.back()}>
    <Icon>close</Icon>
</button>
