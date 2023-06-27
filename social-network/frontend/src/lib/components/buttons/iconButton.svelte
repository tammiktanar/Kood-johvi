<script>
    import { goto } from "$app/navigation"

    export let href = "/"
    export let icon = ""
    export let label = ""

    // If any variables with -- in front are found,
    // append them to the element as css
    $: cssVariables = Object.entries($$props)
        .filter(([key]) => key.startsWith("--"))
        .reduce((css, [key, value]) => `${css}${key}: ${value};`, "")

    // Change the page whenever we are clicked
    const clickHandler = () => {
        goto(`/${href}`)
    }

</script>

<div on:click={clickHandler} style={cssVariables}>
    <img src={icon} alt="An icon for {href}">
</div>

<style lang="scss">
    $--color: var(--color);

    div {
        background-color: $--c0;
        box-shadow: inset 0 0 0 1px $--color;
        border-radius: 50%;
        width: min-content;
        cursor: pointer;
        position: relative;
        width: 64px;
        height: 64px;
        overflow: hidden;
    }

    div:hover {
        box-shadow: 0 0 20px 8px $--color;
    }

    div:before {
        content: "";
        /* background-color: var(--color); */
        background-image: radial-gradient($--color, $--color, $--c0);
        background-size: 5px 5px;
        position: absolute;
        width: 100%;
        height: 100%;
        top: 0px;
        left: -100%;
        transition: all 0.25s ease;
        box-shadow: 0px 0px 20px $--c0 inset;
        border-radius: 50%;
    }

    div:hover:before {
        left: 0px;
    }
    
    img {
        height: 75%;
        margin-left: 12.5%;
        margin-top: 12.5%;
        display: block;
        position: relative;
        transition: all 0.25s ease;
    }

    div:hover img {
        transform: scale(1.1);
        filter: invert(100%);
    }
</style>