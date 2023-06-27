<script>
    export let showLogin = false;
    let email;
    let password;

</script>

{#if showLogin}
    <div class="backdrop" on:click|self>
        <div class="login">
            <form class="loginForm">
                <!-- <h3>Login</h3> -->
                <input type="text" placeholder="E-mail" bind:value={email}>
                <input type="password" placeholder="Password" bind:value={password}>
                <button>Login</button>
            
            </form>
        </div>
    </div>


{/if}


<style>
    .backdrop{
        width: 100%;
        height: 100%;
        position: fixed;
        background: rgba(0, 0, 0, 0.8);
        max-width: 250px;
        max-height: calc(100vh - 20px);
    }
    .login{
        padding: 10px;
        border-radius: 10px;
        max-width: 300px;
        margin: 10% auto;
        text-align: center;
        background: white;
        position: relative;
        top: 30%;
    }
    .loginForm{
        max-width: 300px;
        display: flex;
        flex-direction: column;
    }

</style>