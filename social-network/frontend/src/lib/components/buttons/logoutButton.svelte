<script>
    import BasicButton from "$lib/components/buttons/basicButton.svelte"
    import {goto} from "$app/navigation"
    import {session} from '$app/stores';

    async function handler() {
        const response = await fetch('/api/logout')

        if (response.ok) {
            $session = null
            // Cookie fix
            window.location.href = "/"
            // await goto("/")
        } else {
            console.error("Could not log out!")
        }
    }
</script>

<BasicButton {handler} label="Logout" />

<style>

</style>
