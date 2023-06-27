<script>
    import UserDisplay from "$lib/components/chat/userDisplay.svelte";
    import {io} from "$lib/webSocketConnection.js";
    import {onMount} from 'svelte';
    import {session} from "$app/stores.js";
    import InfiniteScroll from "$lib/components/feed/infiniteScroll.svelte";
    import EmojiSelector from 'svelte-emoji-selector';

    export let active;

    $: isGroup = active.groupID != null;

    let input = "";

    let messages = [];

    export let loadingMore = false;
    let moreToLoad = true;

    onMount(() => {
        io.on("receiveMessage", (message) => {
            if (message.isGroup) {
                if (message.receiver !== active.groupID)
                    return
            } else if (![message.sender, message.receiver].includes(active.userID)) {
                return
            }

            messages = [message, ...messages]
        });
    })

    function sendMessage() {
        if (input === "") return

        io.emit("sendMessage", {
            receiver: active.userID ?? active.groupID,
            content: input,
            isGroup: isGroup
        })

        input = ""
    }

    // const fetchMessages = (target) => {
    //     messages = [];
    //     fetch(`/api/message/history`, {
    //         method: "POST",
    //         body: JSON.stringify({
    //             messageID: messages[0]?.messageID ?? 0,
    //             receiver: active.userID ?? active.groupID,
    //             isGroup: isGroup
    //         })
    //     })
    //         .then(res => res.json())
    //         .then(data => messages = data);
    // }

    $: clearMessages(active)
    async function clearMessages() {
        moreToLoad = true
        messages = []

		    moreToLoad = await loadMessages()
    }

    const loadMessages = async () => {
				if (loadingMore) return true
		    loadingMore = true

		    const lastMessage = messages[messages.length - 1] ?? {
            messageID: messages[0]?.messageID ?? 0,
            receiver: active.userID ?? active.groupID
        }

		    lastMessage.isGroup = isGroup

        let res = await fetch(`/api/message/history`, {
            method: "POST",
            body: JSON.stringify(lastMessage)
        })

		    if (!res.ok) {
				    console.error(res.status, "Failed to fetch messages")
				    return false
		    }

        let data = await res.json()

		    if (data.length === 0) {
		        loadingMore = false
				    return false
		    }
        messages = [...messages, ...data]
		    loadingMore = false
		    return true
    }

    $: inputDisabled = active.userID === 0
</script>

<div id="chat-convo">
	<div id="chat-convo-user">
		<UserDisplay user={active} pointer={false} active={{}} />
	</div>

	<div id="chat-convo-history">
		<!--		<div>-->
		{#each messages as message}
			<div class:isGroup class="message {$session.userID === message.sender ? 'out' : 'in'}"
			     data-message-id="${message.messageID}"
			     data-sender-name="{isGroup ? message.senderData.nickname || message.senderData.firstName : ''}"
			     title="{new Date(message.created).toLocaleString()}">
				{#if active.userID === 0}
					{@html message.content}
				{:else}
					{message.content}
				{/if}
			</div>
		{/each}
		{#if moreToLoad}
		<InfiniteScroll callback={loadMessages} bind:moreToLoad />
		{/if}
		<!--		</div>-->
		<div id="chat-history-top" style="text-align: center"></div>
	</div>

	{#if !inputDisabled}
		<div id="input-row">
			<EmojiSelector on:emoji={(event) => input += event.detail} />
			<form on:submit|preventDefault={sendMessage}>
				<input type="text" id="chat-convo-input" placeholder="Send a message..." bind:value={input} autocomplete="off">
			</form>
		</div>
<!--		<div id="chat-typing" style="display: none;"></div>-->
	{/if}
</div>

<style lang="scss">
		#input-row {
				width: 100%;
				display: flex;
			gap: 3px;
		}

		form {
			flex-grow: 1;
		}

    #chat-convo {
        width: 350px;
        margin-right: 5px;
        margin-top: 10px;
    }

    #chat-convo {
        border-radius: 5px 5px 0 0;

        display: flex;
        flex-direction: column;

        padding: 0.6em;

        background-color: var(--color-primary-dark-1);
        border: 2px solid var(--color-border-dark);
        border-bottom-width: 0;
    }

    #chat-convo-history::-webkit-scrollbar {
        width: 10px;
    }

    #chat-convo-history::-webkit-scrollbar-track {
        background: var(--color-primary);
    }

    #chat-convo-history::-webkit-scrollbar-thumb {
        background: var(--color-primary-light-4);
    }

    #chat-convo-history::-webkit-scrollbar-thumb:hover {
        background: var(--color-primary-light-6);
    }

    #chat-convo > * {
        margin-bottom: 0.5em;
    }

    #chat-typing {
	    margin-bottom: 0;
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
	      align-items: start;
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

    .in {
        background-color: var(--color-primary-dark-2);
        align-self: flex-start;
    }

    .out {
        background-color: var(--color-secondary-dark-5);
        align-self: flex-end;
    }

    .message.in + .message.out,
    .message.out + .message.in {
        margin-top: 0.3em;
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

    form {
        display: flex;
    }

    input {
        width: 100%;
    }

    .isGroup.in::before {
        content: attr(data-sender-name) "";
		    font-size: 0.8em;
	      color: transparentize($--c1, 0.5  )
    }
</style>
