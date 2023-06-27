<script>
    import Title from "$lib/components/header/title.svelte"
    import Navbar from "$lib/components/header/navbar.svelte"
    import LoggenIn from "$lib/components/header/loggedin.svelte"
</script>

<header>
    <!-- <Title /> -->
    <!-- <Navbar /> -->
    <LoggenIn />
</header>

<style lang="scss">
    header {
        background-color: $--c0;
        box-shadow: $--c1 0px 0px 10px;
        padding: 10px;
        display: flex;
        gap: 10px;
        justify-content: flex-end;
        align-items: center;
    }
</style>