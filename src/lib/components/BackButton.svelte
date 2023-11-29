<script lang="ts">
    import Icon from "./Icon.svelte";

    export let clickHandler = () => history.back();
</script>
<button class="absolute -top-2 -right-0 text-gray-500 transition-all
               duration-150 hover:bg-white hover:bg-opacity-5 p-1 rounded-full
               {$$restProps.class}"
        on:click={clickHandler}>
    <Icon>close</Icon>
</button>
