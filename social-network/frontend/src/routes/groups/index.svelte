<script>
    import {session} from '$app/stores';
    import TextArea from "../../lib/components/fields/TextAreaAutosize.svelte";
    import Group from "../../lib/components/groups/group.svelte";
    import {goto} from "$app/navigation"


    export let showCreate = false;
    export let groups = [];
    export let mygroups = [];


    let selector = "showallgroups"

    const defaultImage = "/img/no-profile-picture.jpg";

    let pictureSelected = false;

    const toggleCreate = () => {showCreate = !showCreate};


    async function submitForm(e) {
        const formData = new FormData(e.target)
        const formObject = Object.fromEntries(formData.entries())

        //check if group with same name already exists

        // If the <input> contains only a single file
        let cancontinue = true
        let fileElement = document.querySelector("#profile-picture-input");
        if (fileElement.files.length == 1) {
            const imageFormData = new FormData(
                document.querySelector("#image-form")
            );

            // Filesize check before uploading
            cancontinue = fileElement.files[0].size < 1048576;
            if (!cancontinue){
                alert("File size is too big")
                return
            }
            // Image dimensions check
            let img = new Image();
            let imagePreview = document.querySelector("#image-preview");
            img.src = imagePreview.src;
            cancontinue = img.width == img.height;

            if (!cancontinue){
                alert("File does not fit correct ratio 1:1")
                return
            }
            // ##########################

            // Upload the file
            const uploadResponse = await fetch("/api/file", {
                method: "POST",
                body: imageFormData,
            });

            // Add the received token to the request to be sent to the backend
            let uploadJsonResponse = await uploadResponse.json();
            formObject.image = uploadJsonResponse.token;
        }

        const response = await fetch('/api/group/create', {
        method: 'POST',
        body: JSON.stringify(formObject),
        })

        if (response.ok) {
            const json = await response.json()
            await goto(`/groups/${json.groupID}`)
        }
    }


    function changePreview() {
        let profilePictureInput = document.querySelector(
            "#profile-picture-input"
        );
        let file = profilePictureInput.files[0];
        let imagePreview = document.querySelector("#image-preview");
        let src = file ? URL.createObjectURL(file) : defaultImage;
        imagePreview.src = src;

        pictureSelected = defaultImage != src;
    }

    function deselectProfilePicture(e) {
        let profilePictureInput = document.querySelector(
            "#profile-picture-input"
        );
        profilePictureInput.value = null;
        changePreview();
    }


</script>

<div class="main-container">
    <div class="tab-buttons">
        {#if $session}
            <input class="tab" type="radio" name="selector" bind:group={selector} value={"creategroup"} id="creategroup" selected>
            <label class="tab-label" for="creategroup">
                Create group
            </label>
        {/if}

        <input class="tab" type="radio" name="selector" bind:group={selector} value={"showallgroups"} id="showallgroups">
        <label class="tab-label" for="showallgroups">
            Show all groups
        </label>

        <input class="tab" type="radio" name="selector" bind:group={selector} value={"showmygroups"} id="showmygroups">
        <label class="tab-label" for="showmygroups">
            Show my groups
        </label>
    </div>

    {#if selector == "creategroup"}
        {#if $session}
            <div class="form-container">
                <div class="heading-container">
                    <h3 class="heading-text">Create new group</h3>
                </div>
                <form class="groupForm" on:submit|preventDefault={submitForm}>
                    <div class="row">
                        <input class="textinput" name="name" type="text" placeholder="Group name" required>
                    </div>
                    <div>
                        <TextArea name="about" minRows={2} maxRows={40} placeholder="Describe your group..." required="true"></TextArea>
                    </div>
                    <div class="row">
                        <form id="image-form" hidden>
                            <input id="profile-picture-input" type="file" accept="image/*" name="file" on:change={changePreview}/>
                        </form>
                        <label class="profile-picture-picker" for="profile-picture-input">
                            <img src={defaultImage} alt="" id="image-preview" />
                            Choose picture</label>
                        {#if pictureSelected}
                            <button type="button" class="close-button profile-picture-picker" on:click={deselectProfilePicture}>X</button>
                        {/if}
                    </div>

                    <button class="button" type="submit">Create Group</button>
                </form>
            </div>
        {:else}
            You need to be logged in to create groups!
        {/if}

    {:else if selector == "showallgroups"}

        {#each groups as group}
            <Group group = {group}/>
        {/each}

    {:else if selector == "showmygroups"}
        {#each mygroups as group}
            <Group group = {group}/>
        {/each}
    {/if}
</div>


<!-- <h1>All the groups n stuff.</h1>
<ul>
    <li>Group 1</li>
    <li>Group 2</li>
    <li>Group 3</li>
    <li>Group 4</li>
</ul> -->


<style lang="scss">
    // Profile picture
    .profile-picture-picker {
        // border: $--border-width $--c0-d20 solid;
        display: flex;
        /* background-image: linear-gradient(
            90deg,
            $--c2 0%,
            $--c2-l15 50%,
            $--c2 100%
        ); */
        background-size: 200%;
        // background-position: right;
        transition: 0.2s;
        cursor: pointer;

        color: $--c1;
        font-size: 1.1rem;
        gap: 10px;
        border: none;
        border-radius: $--border-radius;
        padding: 0;
        align-items: center;

        border: $--border-width solid $--c0-d15;
        background-color: $--c0-l5;
        overflow: hidden;
    }

    .profile-picture-picker:hover {
        background-color: $--c0;
        // background-position: left;
    }

    .profile-picture-picker > img {
        height: 48px;
        // border-radius: $--border-radius;
        // border: 2px solid black;
    }

    .upload-button {
        // Resetting styles
        border: none;

        // Additional styles
        background: transparent;
        font-size: 1.1rem;
        padding: 0.5rem 1rem;
        line-height: 0.5;
    }

    .close-button {
        color: $--c2;
        justify-content: center;
        max-width: 25px;
    }

    .tab-buttons {
        display: flex;
        justify-content: space-evenly;
        gap: 5px;
        margin: 5px 10px;
    }

    .tab {
        opacity: 0;
        position: fixed;
        width: 0;
    }

    .tab:checked + label {
        background-color: $--c2-l15;
    }

    .tab-label {
        /* Necessary styles */
        border: none;
        font-size: 24px;
        cursor: pointer;

        /* Custom config */
        padding: 10px 20px;
        border-radius: $--border-radius;
        transition: 0.25s;
        box-sizing: border-box;
        color: $--c1;
        background-color: $--c0;

        width: 100%;
        text-align: center;
    }

    .tab-label:hover {
        background-color: $--c2-l20;
    }

    // Kanguste
    // The container of everything
    .main-container {
      max-width: 600px;
      margin: 0 auto;
      display: flex;
      flex-direction: column;
      gap: 10px;
    }

    // FORM
    .form-container {
      padding: 10px;
      border-radius: $--border-radius;
      background: $--c0-t75;
      border: $--border-width solid $--c0-t75;
      box-shadow: 0 0 15px -10px $--c1;

      // flex
      display: flex;
      flex-direction: column;
      gap: 15px;
    }

    .heading-container {
      background-color: $--c2-l5;
      padding: 5px;
      border-radius: $--border-radius;
    }

    .groupForm {
      display: flex;
      gap: 15px;
      flex-direction: column;
    }

    .heading-text {
      font-size: 2rem;
      text-align: center;
      color: $--c0;
      text-shadow: 0 0 10px gold;
    }

    .row {
      display: flex;
      flex-wrap: wrap;
      gap: 5px;
    }

    .row > * {
      flex-grow: 1;
    }

    // Selection: PRIVATE / PUBLIC
    .main-container {
      max-width: 600px;
      margin: 0 auto;
      display: flex;
      flex-direction: column;
      gap: 10px;
    }

    .selection > input {
      display: none;
    }

    .selection label {
      cursor: pointer;
      text-align: center;

      // Copy-paste
      border: none;
      border-radius: $--border-radius;

      transition: all 1s cubic-bezier(.25, .8, .25, 1);
      font-size: 1.4rem;
      font-weight: bold;
      color: $--c0-l10;
      padding: .5rem 1rem;
    }

    .selection > input:checked + label {
      background-color: $--c2-l15;
    }

</style>
