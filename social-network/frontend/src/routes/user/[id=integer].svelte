<script>
    import BasicButton from "$lib/components/buttons/basicButton.svelte"
    import Post from "$lib/components/feed/post.svelte"
    import {goto, invalidate} from "$app/navigation";
    import {session} from '$app/stores';
    import InfiniteScroll from "$lib/components/feed/infiniteScroll.svelte";

    export let user
    export let posts = []
    export let followers = []
    export let followings = []
    export let access = false
    
    let selection = "posts"

    function loadUserPage(user){

        goto(`/user/${user.userID}`)
    }


    async function follow(){
        await fetch(`/api/user/${user.userID}/follow`, {
                method: 'POST',
        })

        invalidate(`/user/${user.userID}`);
    }

    async function acceptFollow(){
        await fetch(`/api/user/${user.userID}/accept`, {
                method: 'POST',
        })

        user.followInfo.youToMePending = false

        invalidate(`/user/${user.userID}`);
    }

    async function removeFollow(){
        await fetch(`/api/user/${user.userID}/unfollow`, {
                method: 'POST',
        })

        invalidate(`/user/${user.userID}`);
    }

    function editPage(){
        goto("/user/edit")
    }

    async function loadMorePosts() {
        const res = await fetch(`/api/user/${user.userID}/posts?beforeID=${posts[posts.length-1].postID}`)
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

</script>

<div class="container-main">
        <div class="container-left">
            <div class="container-image">
                <img src={user.image} alt="Profile Picture">
            </div>
            {#if $session}
                {#if $session.userID !== user.userID}
                    {#if user.followInfo.meToYou == false}
                        {#if user.followInfo.meToYouPending == false}
                        <div class="follow-div">
                            <input on:click={follow} class="button follow-btn" type="submit" value="Follow">
                        </div>
                        {:else}
                            <div class="follow-div">
                                <input disabled class="button follow-btn" type="submit" value="Pending request">
                            </div>
                        {/if}
                    {:else}
                        <div class="follow-div">
                            <input on:click={removeFollow} class="button follow-btn" type="submit" value="UnFollow">
                        </div>
                    {/if}

                    {#if user.followInfo.youToMePending == true} 
                        <div class="follow-div">
                            <input on:click={acceptFollow} class="button follow-btn" type="submit" value="Accept follow request">
                        </div>
                    {/if}

                {:else}
                    <div class="follow-div">
                        <input on:click={editPage} class="button follow-btn" type="submit" value="Edit Profile">
                    </div>
                {/if}
            {/if}
        </div>
        <div class="container-right">
            <div class="container-header">
                <div class="container-header-left full-width">
                    <h1>{user.firstName} {user.lastName}</h1>

                    <div class="user-div ">
                        <table>
                            <tr>
                                <th>User ID</th>
                                <td>{user.userID}</td>
                            </tr>
                            <tr>
                                <th>First name</th>
                                <td>{user.firstName}</td>
                            </tr>
                            <tr>
                                <th>Last name</th>
                                <td>{user.lastName}</td>
                            </tr>
                            <tr>
                                <th>Nickname</th>
                                <td>{user.nickname}</td>
                            </tr>
                            {#if access}
                                <tr>
                                    <th>Email</th>
                                    <td>{user.email}</td>
                                </tr>
                                <tr>
                                    <th>Created</th>
                                    <td>{new Date(user.created).toLocaleString()}</td>
                                </tr>
                                <tr>
                                    <th>Birthday</th>
                                    <td>{ new Date(user.birthday).toLocaleString()}</td>
                                </tr>
                            {/if}
                        </table>
                    </div>

                </div>

            </div>
            <div class="container-info">
                <h3>About me:</h3>

                {#if access}
                    {#if user.about}
                    <div class="user-div">
                        <p>{user.about}</p>
                    </div>
                    {/if}
                {/if}
            </div>
        </div>
</div>

{#if $session?.userID === user.userID || access}
<div>
    <div class="tab-buttons">
        <input class="tab" type="radio" name="selection" bind:group={selection} value={"posts"} id="posts" selected>
        <label class="tab-label" for="posts">
            Posts
        </label>

        <input class="tab" type="radio" name="selection" bind:group={selection} value={"following"} id="following">
        <label class="tab-label" for="following">
            Following
        </label>
        
        <input class="tab" type="radio" name="selection" bind:group={selection} value={"followers"} id="followers">
        <label class="tab-label" for="followers">
            Followers
        </label>
    </div>

    {#if selection == "posts"}

        <h3>User made posts</h3>
        {#each posts as post}
            <Post post={post} />
        {/each}
        {#if posts.length >= 20}
            <InfiniteScroll callback={loadMorePosts} />
        {/if}

    {:else if selection == "following"}

        <h3>Users {user.firstName} is following</h3>
        {#each followings as following}
            <div class="user-div">
                <a class="user-name-button" on:click={() => loadUserPage(following)}>{following.firstName} {following.lastName}</a>
            </div>
        {/each}

    {:else if selection == "followers"}

        <h3>Users who follow {user.firstName}</h3>
        {#each followers as follower}
            <div class="user-div">
                <a class="user-name-button" on:click={() => loadUserPage(follower)}>{follower.firstName} {follower.lastName}</a>
            </div>
        {/each}
    {/if}

</div>
{/if}

<style lang="scss">

    .full-width {
        width: 100%;
    }

    .user-div {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
    }

    .follow-div {
        grid-column: 1;
        grid-row: 1;
        width: 100%;
        margin-top: 5px;
    }

    .follow-btn {
        width: 100%;
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

    h1, h2, h3, ul, p {
        margin: 0;
    }

    h1 {
        font-weight: bold;
        font-size: 3rem;
    }

    p {
        font-size: 1.2rem;
    }

    h3 {
        font-size: 1.5rem;
    }

    th {
        text-align: left;
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


</style>
