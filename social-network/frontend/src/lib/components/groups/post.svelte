<script>
    export let post
</script>

<div>
    <article>{post.content}</article>
</div>


<style lang="scss">
    div {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
    }


    address {
        font-size: 1.2rem;
    }

    a {
        text-decoration: none;
        color: $--c2-l10;
    }

    a:hover {
        text-decoration: underline;
    }

    article {
        margin-left: 0.5em;
        // border-left: 2px solid black;
        padding: 0.1em 0.2em;
        // background-color: rgba(0, 0, 0, 0.05);
    }
</style>