<script>
		import UserDisplay from "$lib/components/chat/userDisplay.svelte";
		import Convo from "$lib/components/chat/convo.svelte";
    import {session} from "$app/stores.js";
    import { io } from "$lib/webSocketConnection.js";
    import { onMount } from 'svelte';


		let hidden = true;

		let active;

		export let users = [];
		export let groups = [];

		let convoBusy = false

		$: registerGroups(groups)
		function registerGroups(groups) {
				if (groups)
						io.emit("registerGroups", groups.map(group => group.groupID))
		}

		const changeActiveUser = (newUser) => {
				if (convoBusy) {
						return
				}

				if (newUser?.userID != null && newUser.userID !== active?.userID) {
						active = newUser;
				} else if (newUser?.groupID != null && newUser.groupID !== active?.groupID) {
            active = newUser;
				} else {
						active = undefined;
				}
    }

    const clickHideButton = () => {
        hidden = !hidden;

        if (!hidden) {
		        io.emit("getConvos", (res) => {
				        users = res.users
				        groups = res.groups
		        })
        }
    }

    let systemUser = {
		    userID: 0,
		    nickname: "Notifications"
    }
</script>


<div id="chat-wrapper" class:hidden>
	<div id="chat-hide" on:click={clickHideButton}>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
			<path d="M2 5a2 2 0 012-2h7a2 2 0 012 2v4a2 2 0 01-2 2H9l-3 3v-3H4a2 2 0 01-2-2V5z"/>
			<path d="M15 7v2a4 4 0 01-4 4H9.828l-1.766 1.767c.28.149.599.233.938.233h2l3 3v-3h2a2 2 0 002-2V9a2 2 0 00-2-2h-1z"/>
		</svg>
	</div>
	<div id="chat-users">
		<UserDisplay user={systemUser} on:click={() => changeActiveUser(systemUser)} bind:active/>
		<div class="chat-user-group-title">Users</div>
		<div class="chat-user-group">
			{#each users as user}
				<UserDisplay user={user} on:click={() => changeActiveUser(user)} bind:active/>
			{/each}
		</div>

		<div class="chat-user-group-title">Groups</div>
		<div class="chat-user-group">
			{#each groups as group}
				<UserDisplay user={group} on:click={() => changeActiveUser(group)} bind:active/>
			{/each}
		</div>
	</div>

	{#if active}
		<Convo bind:active bind:loadingMore={convoBusy} />
	{/if}
</div>


<style lang="scss">
    #chat-wrapper {
		    --color-text-inverted: #{$--c1};
        --color-text-semi-1: #{$--c1};
        --color-text-semi-4: #{$--c1};
        --color-border-dark: #{transparentize($--c1, 0.75)};
        --color-darken-05: rgba(0, 0, 0, 0.05);
        --color-darken-10: rgba(0, 0, 0, 0.1);
        --color-primary: #{$--c0-l10};
        --color-primary-dark-1: #{$--c0};
        --color-primary-light-1: #{$--c0-l10};
        --color-primary-light-4: #{$--c0-d5};
        --color-primary-light-6: #{$--c0-d10};
        --color-primary-dark-2: #{$--c0-d10};
        --color-secondary-dark-5: #{transparentize($--c2-l15, 0.5)};
        --color-secondary-light-3: #{$--c2-l15};
    }

    #chat-wrapper {
        --height: 400px;

        display: flex;
        flex-direction: row-reverse;

        height: var(--height);
        margin-right: 5px;

        position: fixed;
        bottom: 0;
        right: 0;

        transition: bottom 0.4s;

        user-select: none;

		    z-index: 2;
    }

    #chat-wrapper.hidden {
        bottom: calc(var(--height) * -1);
    }

    #chat-hide {
        cursor: pointer;

        width: 50px;
        height: 50px;
        color: var(--color-text-inverted);
        background-color: var(--color-secondary-light-3);
        border-radius: 50%;

        position: absolute;
        top: -30px;
        right: 5px;

        transition: top 0.5s;

        display: flex;
        justify-content: center;
        align-items: center;

	      z-index: 1;
    }

    #chat-hide > svg {
        width: 70%;
    }

    #chat-wrapper.hidden > #chat-hide {
        top: -55px;
    }

    #chat-users {
        width: 250px;
    }

    #chat-convo {
        width: 350px;
        margin-right: 5px;
        margin-top: 10px;
    }

    #chat-users,
    #chat-convo {
        border-radius: 5px 5px 0 0;

        display: flex;
        flex-direction: column;

        padding: 0.6em;

        background-color: var(--color-primary-dark-1);
        border: 2px solid var(--color-border-dark);
        border-bottom-width: 0;
    }

    #chat-convo.hidden {
        display: none;
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

    .chat-user-group > .chat-user {
        cursor: pointer;
    }

    .chat-user-group > .chat-user:hover {
        background-color: var(--color-darken-05);
    }

    .chat-user-group > .chat-user.selected {
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

    .chat-user-group-title {
        margin: 0.5em 0;
        text-align: center;
        border-bottom: 1px solid var(--color-border-dark);

    }

    .chat-user-group {
        flex-basis: 0;
        flex-grow: 1;
        overflow-y: scroll;
        overflow-x: hidden;
        scrollbar-width: thin;

        display: flex;
        flex-direction: column;

        transition: flex-grow 0.25s;
    }

    .chat-user-group:hover {
        flex-grow: 6.2;
    }

    .chat-user-group::-webkit-scrollbar,
    #chat-convo-history::-webkit-scrollbar {
        width: 10px;
    }

    .chat-user-group::-webkit-scrollbar-track,
    #chat-convo-history::-webkit-scrollbar-track {
        background: var(--color-primary);
    }

    .chat-user-group::-webkit-scrollbar-thumb,
    #chat-convo-history::-webkit-scrollbar-thumb {
        background: var(--color-primary-light-4);
    }

    .chat-user-group::-webkit-scrollbar-thumb:hover,
    #chat-convo-history::-webkit-scrollbar-thumb:hover {
        background: var(--color-primary-light-6);
    }

    #chat-convo > *:not(:last-child) {
        margin-bottom: 0.5em;
    }

    #chat-convo-history {
        padding: 0.3em;

        background-color: var(--color-primary-light-1);
        border-radius: 5px;

        overflow-y: scroll;
        overflow-x: hidden;
        scrollbar-width: thin;

        flex-basis: 0;
        flex-grow: 1;

        display: flex;
        flex-direction: column-reverse;

        height: 300px;
    }

    #chat-convo-history > div {
        display: flex;
        flex-direction: column;
    }

    .message {
        margin: 0.1em;
        padding: 0.5em;

        max-width: 75%;

        border-radius: 5px;

        user-select: text;

        word-break: break-word;
    }

    .message.unconfirmed {
        color: rgba(0, 0, 0, 0.5);
    }

    .message.in {
        background-color: var(--color-primary-dark-2);
        align-self: flex-start;
    }

    .message.out {
        background-color: var(--color-secondary-dark-5);
        align-self: flex-end;
    }

    .message.in + .message.out,
    .message.out + .message.in {
        margin-top: 0.3em;
    }

    .clickthrough {
        pointer-events: none;
    }

    #chat-history-top {
        width: 100%;
        min-height: 1px;
    }

    #chat-typing {
        position: absolute;
        left: 20px;
        bottom: 53px;
        font-size: 0.7em;
        color: var(--color-text-semi-1);
    }

</style>
