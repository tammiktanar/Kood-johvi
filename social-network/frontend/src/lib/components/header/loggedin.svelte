<script>
    import {session} from '$app/stores';
    import ProfileImage from "$lib/components/profileimage.svelte"
    import BasicButton from "$lib/components/buttons/basicButton.svelte"
    import LogoutButton from "$lib/components/buttons/logoutButton.svelte"

    // let user = {
    //     nickname: "Kanguste",
    //     image: "https://picsum.photos/256"
    // }


</script>

<div class="outer">
    {#if $session}
        <div class="outer">
            <div class="inner start">
                <ProfileImage src={$session.image ? "/api/file/"+$session.image : "/img/no-profile-picture.jpg"} />
                <a href="{'/user/'+$session.userID}">{$session.firstName}</a>
            </div>
            <LogoutButton/>
        </div>
    {:else}
        <div class="inner stretch">
            <BasicButton href="/login" label="Login" />
            <BasicButton href="/register" label="Register" />
        </div>
    {/if}
</div>

<style lang="scss">
    .outer {
        display: flex;
        flex-direction: column;
        align-items: stretch;
        gap: 10px;
    }

    .inner {
        display: flex;
        align-items: center;
        gap: 10px;
    }

    .inner.start {
        justify-content: start;
    }

    .inner.stretch {
        justify-content: center;
    }

    .loggedin {
        font-size: 20px;
        color: $--c1;
    }

    a {
        color: $--c2;
        font-size: 1.4rem;
        text-decoration: none;
    }

    a:hover {
        text-decoration: underline;
    }
</style>
