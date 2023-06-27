<script>
    import ProfileImage from "$lib/components/profileimage.svelte"

    export let label = "<undefined>"
    export let src = "https://picsum.photos/256"

</script>

<div title={label}>
    <ProfileImage {src} />
    <span>{label}</span>
</div>

<style lang="scss">
    div {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 5px;
        border-radius: $--border-radius;
        transition: .25s;
        cursor: pointer;
    }

    div:hover {
        background-color: $--c2-l20;
    }

    span {
        font-size: 1.5rem;
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
    }
</style>