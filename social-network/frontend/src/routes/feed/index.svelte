<script>
    import Post from "$lib/components/feed/post.svelte"
    import Addpost from "$lib/components/addpost.svelte";
    import {goto} from "$app/navigation";
    import InfiniteScroll from "$lib/components/feed/infiniteScroll.svelte";
    import {session} from "$app/stores";

    export let posts = []
    let selector = "feed"
    let initial_launch = true

    async function createPost(formObject) {
        const response = await fetch('/api/post/create', {
            method: 'POST',
            body: JSON.stringify(formObject),
        })

        if (!response.ok) {
            console.error("Could not create post: " + await response.text())
            return
        }

        const json = await response.json()
        await goto(`/post/${json.postID}`)
    }

    function chooseURL(selected) {
        switch (selected) {
            default:
            case "feed":
                return "/api/post/all"
            case "following":
                return "/api/post/all/following"
            case "groups":
                return "/api/post/all/groups"
        }
    }

    let loadingMore = false;

    async function updatePosts(selectedField) {
        if (initial_launch) {
            initial_launch = false;
            return
        }
        loadingMore = true

        posts = []
        const feed_response = await fetch(chooseURL(selectedField), {
            method: "GET"
        })
        if (!feed_response.ok) {
            loadingMore = false
            throw new Error(await feed_response.text())
        }

        posts = await feed_response.json()
        loadingMore = false
    }

    async function loadMorePosts() {
        if (loadingMore) return

        const res = await fetch(`${chooseURL(selector)}?beforeID=${posts[posts.length-1].postID}`)
        if (!res.ok) {
            console.error(res.status, "Error loading more posts")
            return false
        }

        const data = await res.json()

        if (data.length === 0) {
            return false
        }

        if (loadingMore) return

        posts = [...posts, ...data]
        return true
    }

    $: updatePosts(selector)
</script>

<div>
    <div>
        <div class="tab-buttons">
            <input class="tab" type="radio" name="selection" bind:group={selector} value={"feed"} id="feed" selected>
            <label class="tab-label" for="feed">
                Feed
            </label>

            {#if $session != null}
                <input class="tab" type="radio" name="selection" bind:group={selector} value={"following"} id="following">
                <label class="tab-label" for="following">
                    Followings
                </label>

            <input class="tab" type="radio" name="selection" bind:group={selector} value={"groups"} id="groups">
            <label class="tab-label" for="groups">
                Groups
            </label>
            {/if}
        </div>
    </div>
    <div>
        {#if selector != "groups" && $session != null}
            <h2>Make a post</h2>
            <Addpost handler={createPost} submitButtonText="Post"/>
        {/if}

        {#each posts as post}
            <Post post={post} onfeed={true} />
        {/each}
        {#if posts.length >= 20}
            <InfiniteScroll callback={loadMorePosts} />
        {/if}

    </div>
</div>

<style lang="scss">
    div {
        display: flex;
        flex-direction: column;
        height: 100%;
    }

    
    .tab-buttons {
        display: flex;
        justify-content: space-evenly;
        flex-direction: row;
        gap: 5px;
        margin: 5px 10px;
    }

    .tab {
        opacity: 0;
        position: fixed;
        width: 0;
    }

    .tab:checked + label {
        background-color: $--c2-l15;
    }

    .tab-label {
        /* Necessary styles */
        border: none;
        font-size: 24px;
        cursor: pointer;

        /* Custom config */
        padding: 10px 20px;
        border-radius: $--border-radius;
        transition: 0.25s;
        box-sizing: border-box;
        color: $--c1;
        background-color: $--c0;

        width: 100%;
        text-align: center;
    }

    .tab-label:hover {
        background-color: $--c2-l20;
    }

</style>
