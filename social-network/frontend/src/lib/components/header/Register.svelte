<script>
    export let showRegister = false;
    let firstName;
    let lastName;
    let email;
    let password;
    let passwordConfirm;
</script>

{#if showRegister}
    <div class="backdrop" on:click|self>
        <div class="register">
            <form class="registerForm" method="post">
                <input type="text" placeholder="First name" bind:value={firstName} required>
                <input type="text" placeholder="Last name" bind:value={lastName} required>
                <input type="email" placeholder="E-mail" bind:value={email} required>
                <input type="password" placeholder="Password" bind:value={password} required>
                <input type="password" placeholder="Confirm password" bind:value={passwordConfirm} required>
                <button type="submit">Register</button>
            
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
    .register{
        padding: 10px;
        border-radius: 10px;
        max-width: 300px;
        margin: 10% auto;
        text-align: center;
        background: white;
        position: relative;
        top: 30%;
    }
    .registerForm{
        max-width: 300px;
        display: flex;
        flex-direction: column;
    }

</style>