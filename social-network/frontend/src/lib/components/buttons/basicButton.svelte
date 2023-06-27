<script>
    import {goto} from "$app/navigation"

    export let label = "Unlabeled"
    export let href = "/"

    export let handler = async () => {
        await goto(href)
    }
</script>

<button on:click={handler} title={label}>
    {label}
</button>

<style lang="scss">
    button {
        /* Necessary styles */
        font-size: 24px;
        cursor: pointer;

        /* Custom config */
        padding: 10px 20px;
        border-radius: $--border-radius;
        transition: 0.25s;
        color: $--c1;
        background-color: $--c2-t30;
        border: $--border-width transparent solid;
    }

    button:hover {
        background-color: transparent;
        border-color: $--c2-t30;
    }
</style>
