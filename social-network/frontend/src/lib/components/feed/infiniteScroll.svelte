<script>
    import {onDestroy, onMount} from "svelte";

    export let moreToLoad = true
    let element

    export let callback = () => {
    }

    let middleFunc = async (entries) => {
        if (!entries[0].isIntersecting) {
            return
        }

        moreToLoad = await callback()
    }

    let observer;

    onMount(() => {
        observer = new IntersectionObserver(middleFunc, {
            root: null,
            rootMargin: '0px',
            threshold: 0.1
        });

        observer.observe(element)
    })

    onDestroy(() => {
        observer?.disconnect()
    })

</script>

{#if moreToLoad}
	<span bind:this={element}>Loading more!</span>
{/if}

<style>
    span {
        margin: 0 auto;
    }
</style>
