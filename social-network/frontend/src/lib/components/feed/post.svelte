<script>
    export let post
    export let onfeed = false
    let postDate = new Date(post.created).toLocaleString()
    let commentAmount = 0

    function openImage(e) {
        window.open(e.target.src, '_blank');
    }
</script>


    <div class="post-div">
        <div>
            <address><a rel="author" href="/user/{post.author.userID}">{post.author.firstName}</a> posted {#if post.group != null} in <a href="/groups/{post.groupID}">{post.group.name||"group_name"}</a> {/if} at {postDate}:</address>
            <article>
                <p>
                    {post.content}
                </p>
                {#if post.images !== ""}
                    <hr/>
                    <img src="/api/file/{post.images}" alt="{post.images}" on:click={openImage}/>
                {/if}
            </article>

        </div>
        
        {#if onfeed}
            <a class="read-more" href="/post/{post.postID}">read comments... </a>
        {/if}
    </div>


<style lang="scss">
    .read-more{
        margin: 5px 10px;
    }

    .post-div {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
    }

    address {
        font-size: 1.2rem;
        opacity: 0.8;
    }

    a {
        text-decoration: none;
        color: $--c2-l5;
        font-weight: bold;
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

    img {
        max-height: 200px;
        cursor: pointer;
    }

    p {
        width: 100%;
        border-radius: 5px;
        //background-color: transparentize($--c0, 0.5);
        //padding: 0.2em 1em;
        font-size: 1.1rem;
    }

    hr {
        margin: 0.5em 0;
    }
</style>
