<script>
export let handler
export let groupID = undefined
let value

async function submitForm(e) {
    const formData = new FormData(e.target);
    const formObject = Object.fromEntries(formData.entries());
    formObject["time"] = new Date(value).toISOString();
    formObject["groupID"] = groupID;
    handler(formObject);


}

</script>

<div class="container">
    <form class="form" on:submit|preventDefault={submitForm}>
        <div class="container">
            <div class="fields">
                <input  class="field textinput" name="title" type="text" placeholder="Event title" required/>
            </div>

            <div class="fields">
                <input class="field textinput" bind:value name="time" type="datetime-local" required/>
            </div>

            <div class="fields">
                <textarea  class="field textinput" name="about" placeholder="Event description" required/>
            </div>
            
            <div class="fields">
                <button class="button field" type="submit">Add event</button>
            </div>

        </div>
    </form>
</div>




<style lang="scss">
    .form {
        margin: 0;
    }

    .fields {
        margin-top: 5px;
        display: flex;
        justify-content: space-evenly;
    }

	textarea {
		font-family: inherit;
		padding: 0.5em;
		box-sizing: border-box;
		border: 2px solid $--c0-d15;
		border-radius: 5px;
		line-height: 1.2;
		overflow: hidden;
        resize: none;
	}

    .field {
        width: 100%;
    }

    .field:focus {
        border-color: black;
    }

    .container {
        width: 100%;
    }
</style>
