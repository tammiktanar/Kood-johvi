<script>
    import Sidebar from "$lib/components/sidebar.svelte";
    import Tabs from "$lib/components/selectionlist/tabs.svelte";
    import LoggedIn from "$lib/components/header/loggedin.svelte";
    import HoverSlide from "$lib/components/buttons/hoverSlide.svelte";
    import Chat from "$lib/components/chat/chat.svelte";
    import {session} from "$app/stores.js";
</script>

<div class="container">
    <Sidebar>
        <div class="sidebar">
            <div class="sidebar-section">
                <HoverSlide refer="feed" icon="/icons/rss.svg" label="Feed" />
                {#if $session}
                <HoverSlide
                    refer="{'user/'+$session.userID}"
                    icon="/icons/account.svg"
                    label="Profile"
                />
                {/if}
                <HoverSlide
                    refer="groups"
                    icon="/icons/account-group.svg"
                    label="Groups"
                />
                <HoverSlide
                    refer="about"
                    icon="/icons/information.svg"
                    label="About"
                />
            </div>
            <div class="sidebar-section">
                <LoggedIn />
            </div>
        </div>
    </Sidebar>

    <main>
        <div id="content-wrapper">
            <slot />
        </div>
    </main>

    {#if $session}
        <Chat/>
    {/if}
</div>

<style lang="scss">
    .sidebar {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        height: 100%;
    }

    .sidebar-section {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .container {
        display: flex;
        height: 100vh;
        // max-width: 1500px;
        margin: 0 auto;
    }

    main {
        flex-grow: 1;
        background-image: linear-gradient(180deg, $--c0-d15 0%, $--c0-l5 100%);
        overflow-y: auto;
        box-shadow: inset 0 0 5px 0 $--c0-d20;
    }

    #content-wrapper {
        padding: 10px;
        overflow: visible;

        max-width: 1000px;
        margin: 0 auto;
    }

    :global(body) {
        background-color: $--c0-d5;
    }
</style>
