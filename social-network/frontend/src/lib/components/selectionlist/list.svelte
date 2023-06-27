<script>
    import ListItem from "$lib/components/selectionlist/listitem.svelte"

    export let labels = []
</script>

<ul>
    {#each labels as label (label)}
    <li>
        <ListItem {label} />
    </li>
    {/each}
</ul>

<style>
    ul {
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    li {
        list-style-type: none;
    }
</style>
