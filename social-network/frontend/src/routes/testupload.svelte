<script>
    let token = ""

    async function submitForm(e) {
        const formData = new FormData(e.target)

        const response = await fetch('/api/file', {
            method: 'POST',
            body: formData,
        })

        if (response.ok) {
            const json = await response.json()
            token = json["token"]
        }
    }
</script>

<div>
	<form class="loginForm" on:submit|preventDefault={submitForm}>
		<input accept="image/png, image/jpeg" type="file" name="file"/>
		<button type="submit">Upload</button>
	</form>
	{#if token}
		<span><label>Received token: </label>{token}</span>
		<img src="/api/file/{token}" alt="">
	{/if}
</div>

<style>
    div, form {
        display: flex;
        flex-direction: column;
		    align-items: center;
    }
</style>
