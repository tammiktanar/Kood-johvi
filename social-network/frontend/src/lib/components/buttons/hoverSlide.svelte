<script>
    import { goto } from "$app/navigation";

    export let refer = "/";
    export let icon = "";
    export let label = "Button";

    const handleClick = () => {
        goto(`/${refer}`);
    };
</script>

<div class="container" on:click={handleClick}>
    <div class="image">
        <img src={icon} alt="" />
    </div>
    <div class="label">
        {label}
    </div>
</div>

<style lang="scss">
    .container {
        display: flex;
        align-items: center;
        gap: 10px;
        border-radius: $--border-radius;
        cursor: pointer;
        position: relative;
    }

    .container:before {
        content: "";
        background-image: linear-gradient(180deg, $--c2-l15 0%, $--c2-l5 100%);
        box-shadow: 0 0 10px -5px $--c1-l5;

        width: 60px;
        height: 100%;
        position: absolute;
        z-index: -1;
        border-radius: $--border-radius;
        transition: 0.2s;
    }

    .container:hover.container:before {
        width: 100%;
    }

    .image {
        padding: 10px;
    }

    img {
        width: 40px;
        height: 40px;
    }

    .label {
        font-size: 1.6rem;
        margin: 10px;
        transition: 0.2s;
    }
</style>
