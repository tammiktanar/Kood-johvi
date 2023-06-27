<script>
    import {goto} from "$app/navigation"

    async function submitForm(e) {
        const formData = new FormData(e.target)
        const formObject = Object.fromEntries(formData.entries())

        // if (formObject.password !== formObject.passwordConfirm) {
        //     passMismatch = true
        //     return
        // }

        const response = await fetch('/api/group/create', {
        method: 'POST',
        body: JSON.stringify(formObject),
        })

        if (response.ok) {
            const json = await response.json()
            await goto(`/groups/${json.groupID}`)
        }
    }


</script>
<h3>Create new group</h3>
<form class="groupForm" on:submit|preventDefault={submitForm}>
    <input name="Group name" type="text" placeholder="Group name" required>
    <input name="Group description" type="text" placeholder="Group description" required>
    <input name="type" type="radio" id="public" value="public" checked>
    <input name="type" type="radio" id="private" value="private">
    <button type="submit">Create Group</button>
</form>

<style>

</style>
