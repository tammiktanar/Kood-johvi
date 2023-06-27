<script>
    import BasicButton from "$lib/components/buttons/basicButton.svelte"
</script>

<nav>
    <!-- Navbar shall just include lots of buttons -->

    <BasicButton text="User" refer="user" />
    <BasicButton text="Follower" refer="followers" />
    <BasicButton text="Groups" refer="groups" />
</nav>

<style lang="scss">
    nav {
        display: flex;
        gap: 5px;
    }
</style>