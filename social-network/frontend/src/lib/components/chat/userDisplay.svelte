<script>
	export let user;
	export let pointer = true;
	export let active;
	// export let active = undefined;

	$: isGroup = user?.groupID != null;
	// $: activeBool = isGroup ? user?.groupID === active?.groupID : user?.userID === active?.userID;
</script>

<div class="chat-user" on:click class:online={user.online} class:pointer class:selected={user === active}>
	<img class="chat-user-image clickthrough" src="{user.userID === 0 ? '/img/system-user.png' : user.image ? '/api/file/'+user.image : '/img/no-profile-picture.jpg'}" alt="">
	<div class="chat-user-names clickthrough">
		{#if !isGroup}
			<span class="chat-username">{user.nickname ?? ''}</span>
			<span class="chat-fullname">{user.firstName ?? ''} {user.lastName ?? ''}</span>
		{:else}
			<span class="chat-username">{user.name ?? ''}</span>
		{/if}
	</div>
</div>

<style>
    .clickthrough {
        pointer-events: none;
    }

    .chat-user {
        display: flex;
        padding: 2px 5px;
        margin: 0 3px;
        border-radius: 5px;

        position: relative;
    }

    .chat-user.online::after {
        /* Super shitty implementation here, but it'll do for now */
        display: block;

        border-radius: 100%;

        position: absolute;
        bottom: 0.2em;
        left: 2em;

        content: '';
        height: 0.6em;
        width: 0.6em;
        background-color: #0aee0a;
    }

    .pointer {
        cursor: pointer;
    }

    .pointer:hover {
        background-color: var(--color-darken-05);
    }

    .pointer.selected {
        background-color: var(--color-darken-10);
    }

    .chat-user-image {
        border-radius: 50%;
        height: 2.2em;
        background-color: transparent;
        /*   border: 2px solid black; */
        margin-right: 5px;
    }

    .chat-user-names {
        display: flex;
        flex-direction: column;
        justify-content: center;

        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .chat-username {
        font-size: 1em;
    }

    .chat-fullname {
        font-size: 0.8em;
        color: var(--color-text-semi-4);
    }
</style>
