<script>
    import { goto, invalidate } from "$app/navigation";
    import { session } from "$app/stores";
    import Addpost from "$lib/components/addpost.svelte";
    import Addevent from "$lib/components/addevent.svelte";
    import Post from "$lib/components/feed/post.svelte";
    import Event from "$lib/components/groups/event.svelte";
    import MultiSelect from "$lib/components/multiselect.svelte";
    import InfiniteScroll from "$lib/components/feed/infiniteScroll.svelte";

    export let group = [];
    export let posts = [];
    export let members = [];
    export let events = [];
    export let followList = [];
    export let request_state = group.pendingRequest ? "Accept invite" : "Request to join";

    let inGroup = group.includesMe
    let invite_people = [];


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

    async function createEvent(formObject) {
        const response = await fetch('/api/event/create', {
            method: 'POST',
            body: JSON.stringify(formObject),
        })


        if (!response.ok) {
            console.log(formObject);
            console.error("Could not create event: " + await response.text())
            return
        }

        const json = await response.json()
        await goto(`/event/${json.eventID}`)
    }

    function loadUserPage(user){

        goto(`/user/${user.userID}`)
    }

    const intermediate = (e) => {
        const formData = new FormData(e.target)
        const formObject = Object.fromEntries(formData.entries())
        sendInvites(formObject)
    }

    const intermediate_join = async () => {
        const user_response = await fetch(`/api/group/${group.groupID}/join`, {
            method: 'POST',
        })

        if (!user_response.ok) {
            console.error("Could not join group: " + await user_response.text())
            return
        }
        request_state = "Accept invite" 
    }

    async function sendInvites(formObject) {

        if (formObject) {
            invite_people.forEach( async (userID) => {


                const response = await fetch(`/api/group/${group.groupID}/invite/${userID}`, {
                    method: 'POST',
                })

                if (!response.ok) {
                    console.error("Could not invite user: " + await response.text())
                    return
                }
            })



            goto(`/groups/${group.groupID}`);
            invalidate(`/groups/${group.groupID}`);
            alert("Invite sent")

        }
    }

    async function loadMorePosts() {
        const res = await fetch(`/api/group/${group.groupID}/posts?beforeID=${posts[posts.length-1].postID}`)
        if (!res.ok) {
            console.error(res.status, "Error loading more posts")
            return false
        }

        const data = await res.json()

        if (data.length === 0) {
            return false
        }

        posts = [...posts, ...data]
        return true
    }

    let selector = "posts"
    let group_image = ""
    if (group.image != null) {
        group_image = `/api/file/${group.image}`
    }
</script>

<div class="container-main">
    <div class="container-left">
        <div class="container-image">
            <img src={group_image  || "/img/no-profile-picture.jpg"} alt="Profile Picture">
        </div>
    </div>
    <div class="container-right">
        <div class="container-header">
            <div class="container-header-left full-width">
                <h1>{group.name}</h1>

                <div class="group-div ">
                    {group.about || "Group about me"}
                </div>

            </div>

        </div>
    </div>
</div>


<div class="tab-buttons">

    <input class="tab" type="radio" name="selector" bind:group={selector} value={"posts"} id="posts" selected>
    <label class="tab-label" for="posts">
        Posts
    </label>

    <input class="tab" type="radio" name="selector" bind:group={selector} value={"events"} id="events">
    <label class="tab-label" for="events">
        Events
    </label>

    <input class="tab" type="radio" name="selector" bind:group={selector} value={"members"} id="members">
    <label class="tab-label" for="members">
        Members
    </label>
</div>

{#if selector == "posts"}
    {#if inGroup}
        <h2>Make a post</h2>
        <Addpost handler={createPost} groupID={group.groupID} submitButtonText="Post"/>
    {/if}

    {#each posts as post}
        <Post post={post} onfeed={true} />
    {/each}
    {#if posts.length >= 20}
        <InfiniteScroll callback={loadMorePosts} />
    {/if}


{:else if selector == "events"}
    <div class="container">
        {#if group.includesMe}
            <h2>Create new event</h2>

            <div class="user-div">
                <Addevent handler={createEvent} groupID={group.groupID}/>
            </div>
        {/if}

        {#each events as event}
            <Event event={event} includesMe={group.includesMe}></Event>
        {/each}
    </div>
{:else if selector == "members"}
    {#if $session != null}
        <div class="invite-user">
            {#if inGroup}
                <form class="invite-form" on:submit|preventDefault={intermediate}>
                    <div class="invite-div">
                        <div class="multi-select-div">
                            <MultiSelect id='lang' bind:value={invite_people}>
                                {#each followList as follower}<option value={follower.userID}>{follower.firstName} {follower.lastName}</option>{/each}
                            </MultiSelect>
                        </div>
                        <input type='submit' class="button" value="invite" >
                    </div>
                </form>
            {:else}
                <form on:submit|preventDefault={intermediate_join}>
                    <input type='submit' class="button request-button" value={request_state} >
                </form>

            {/if}
        </div>
    {/if}
    {#each members as user}
        <div class="user-div">
            <a class="user-name-button" on:click={() => loadUserPage(user)}>{user.firstName} {user.lastName}</a>  
            {#if group.ownerID == user.userID} 
                <h2>Owner</h2>
            {/if}
        </div>
    {/each}
{/if}

<style lang="scss">

    .invite-form {
        width: 100%;
    }

    .invite-div {
        display: flex;
        gap: 5px;
    }

    .multi-select-div {
        flex-grow: 1;
    }

    .invite-user {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
        align-items: baseline;
        display: flex;
    }


    .user-field {
        font-family: inherit;
		padding: 0.1em;
		box-sizing: border-box;
		border: 2px solid $--c0-d15;
		border-radius: 5px;
		line-height: 1.2;
        height: 41px;
		overflow: hidden;
        resize: none;
    }

    .user-div {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
        display: flex;
        justify-content: space-between;
    }

    .user-name-button:hover {
        color: $--c2-d5;
        cursor: pointer;
    }

    .container-main {
        margin: 10px;
        display: flex;
        gap: 20px;
    }

    .container-image {
        border-radius: $--border-radius;
        overflow: hidden;
        border: $--c1 $--border-width solid;
        box-sizing: border-box;
    }

    img {
        display: block;
        object-fit: cover;
        width: 256px;
        height: 256px;
    }

    .container-right {
        display: flex;
        flex-direction: column;
        flex-grow: 1;
        gap: 20px;
    }

    .container-header {
        display: flex;
        justify-content: space-between;
    }


    .group-div {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
    }

    .tab-buttons {
        display: flex;
        justify-content: space-evenly;
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

    .request-button {
        margin-left: 0.5em;
    }
</style>
