<script>
    import IconButton from "$lib/components/buttons/iconButton.svelte"
</script>

<div>
    <IconButton refer="user" --color="#C82647" src="/icons/account.svg" />
    <IconButton refer="followers" --color="rgb(255, 190, 116)" src="/icons/account-multiple.svg" />
    <IconButton refer="groups" --color="#16A085" src="/icons/account-group.svg" />
</div>

<style lang="scss">
    div {
        display: flex;
        flex-direction: column;
        width: min-content;
        height: min-content;
        box-shadow: $--c1 0px 0px 14px;
        padding: 12px;
        gap: 10px;
        border-radius: $--border-radius;
        position: relative;
        
        /* Background stripes */
        background-color: $--c0;
        border: 2px solid $--c1;
    }

    div:before, div:after {
        content: "";
        background-color: $--c0;
        width: 10px;
        height: 5px;
        position: absolute;
        display: block;
        left: -12px;
        bottom: 20px;
        z-index: 10;
    }

    div:after {
        top: 20px;
    }
</style>