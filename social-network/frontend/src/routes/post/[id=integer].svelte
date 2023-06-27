<script>
    import Post from "$lib/components/feed/post.svelte"
    import Comment from "$lib/components/feed/comment.svelte"


    import Addpost from "$lib/components/addpost.svelte";
    import {goto, invalidate} from "$app/navigation";
    import {page} from '$app/stores';

    export let post
    let commentCreateContent

    async function createPost(formObject) {
        const response = await fetch(`/api/post/${post.postID}/comment/create`, {
            method: 'POST',
            body: JSON.stringify(formObject),
        })

        if (!response.ok) {
            console.error("Could not create comment: " + await response.text())
            return
        }

        const json = await response.json()
        await goto(`/post/${json.postID}`)

        let commentResponse = await fetch(`/api/post/${post.postID}/comment/all`, {
            method: 'GET'
        })

        if (!commentResponse.ok) {
            return {
                status: commentResponse.status
            }
        }

//        post.comments = await commentResponse.json()
        commentCreateContent = ""
        invalidate($page.url.href)
    }

    function openGroup(groupID) {
        goto(`/groups/${groupID}`)
    }
</script>

{#if post.groupID != null}
	<input on:click={openGroup(post.groupID)} class="button open-group-button " type="submit" value="Open group">
{/if}

<Post post={post}/>

{#each post.comments as comment}
	<Comment comment={comment}></Comment>
{/each}

<div class="comment-creation-div">
	<Addpost handler={createPost} postID={post.postID} submitButtonText="Comment" bind:value={commentCreateContent}/>
</div>


<style lang="scss">
  .open-group-button {
    margin-left: 0.4em;
  }
</style>
