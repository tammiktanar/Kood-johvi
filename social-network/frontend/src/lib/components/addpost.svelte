<script>
    import { session } from "$app/stores";
    import TextArea from "./fields/TextAreaAutosize.svelte";
    import MultiSelect from "./multiselect.svelte";

    // handler is a function that receives the form's data as
    export let handler
    export let postID = undefined
    export let groupID = undefined
    export let submitButtonText = "Submit"
    export let value = ""
    export let squareImg = false;

    let followList = []
    let followListReady = false

    let privacy = "public"
    let manualPrivacy = []

    let minRows = 4
    
    if (postID) {
        minRows = 2
    }

    if (groupID){
        minRows = 2
    }

    async function getFollowers(id) {
        let res = await fetch(`/api/user/${id}/followers`, {
            method: "GET"
        });

        if (res.ok == true){
            res = await res.json()
        }

        return res
    }

    async function updateFollowerList(){
        followList = await getFollowers($session.userID);
        followListReady = true
    }

    const defaultImage = "/img/no-profile-picture.jpg";

    let pictureSelected = false;
    // For live preview of the image (without uploading)
    function changePreview() {
        let postPictureInput = document.querySelector(
            "#post-picture-input"
        );
        let file = postPictureInput.files[0];
        let imagePreview = document.querySelector("#image-preview");
        let src = file ? URL.createObjectURL(file) : defaultImage;
        imagePreview.src = src;

        pictureSelected = defaultImage != src;
    }

    function deselectPostPicture (e) {
        let postPictureInput = document.querySelector(
            "#post-picture-input"
        );
        postPictureInput.value = null;
        changePreview();
    }

    const intermediate = async (e) => {
        const formData = new FormData(e.target)
        const formObject = Object.fromEntries(formData.entries())
        let continuePostCreation = true

        if (groupID){ // groups
            formObject["groupID"] = groupID
            formObject["privacy"] = "public"

        } else if (!postID) { // normal posts
            formObject["privacy"] = privacy

            if (privacy == "manual") {
                formObject["allowedUsers"] = manualPrivacy.map(str => {
                    return Number(str);
                });
            }
        }

        // If the <input> contains only a single file
        let fileElement = document.querySelector("#post-picture-input");

        if (fileElement.files.length == 1) {
            const imageFormData = new FormData(
                document.querySelector("#image-form")
            );

            // Filesize check before uploading
            continuePostCreation = fileElement.files[0].size < 1048576;

            if (continuePostCreation) {
                // Image dimensions check
                let img = new Image();
                let imagePreview = document.querySelector("#image-preview");
                img.src = imagePreview.src;

                // continuePostCreation = img.width === img.height;

                if (continuePostCreation) {
                    // Upload the file
                    const uploadResponse = await fetch("/api/file", {
                        method: "POST",
                        body: imageFormData,
                    });

                    // Add the received token to the request to be sent to the backend
                    let uploadJsonResponse = await uploadResponse.json();
                    formObject["images"] = uploadJsonResponse.token;
                }
            }
        }

        if (continuePostCreation){
            handler(formObject)
        } else {
            console.log("Errored")
        }
    }
</script>

<div>
    <form on:submit|preventDefault={intermediate}>
        <TextArea name="content" minRows={minRows} maxRows={40} placeholder="Write something..." required="true" bind:value={value}></TextArea>
        
        {#if postID } <!-- If it's a comment -->
            <input name="postID" value={postID} hidden disabled/>

        {:else if groupID}  <!-- If it's a group post -->
            <input name="groupID" value={groupID} hidden disabled/>

        {:else} <!-- If it's a normal post -->

            <select class="select-field" bind:value={privacy} on:click={updateFollowerList} on:change={updateFollowerList}>
                <option value="public" selected> public</option>
                <option value="private">private</option>
                <option value="manual">manual</option>
            </select>

            {#if privacy == "manual" && followListReady == true}
                <h4>Select followers to see your post</h4>
                <MultiSelect id='lang' bind:value={manualPrivacy}>
                    {#each followList as follower}<option value={follower.userID}>{follower.firstName} {follower.lastName}</option>{/each}
                    
                </MultiSelect>
            {/if}
        
        {/if}
        <div class="row">
            <form id="image-form" hidden>
                <input
                    id="post-picture-input"
                    type="file"
                    accept="image/*"
                    name="file"
                    on:change={changePreview}
                />
            </form>
            <label
                class="post-picture-picker"
                for="post-picture-input"
            >
                <img src={defaultImage} alt="" id="image-preview" />
                Choose picture</label
            >
            {#if pictureSelected}
                <button
                    type="button"
                    class="close-button post-picture-picker"
                    on:click={deselectPostPicture}>X</button
                >
            {/if}
        </div>

        <input id="submit-content" class="button" type="submit" value={submitButtonText}>
    </form>
</div>

<style lang="scss">
    div {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 5px;
        margin: 5px 10px;
    }

    input {
        background-color: $--c2-l20;
        border-radius: 5px;
        border-width: 0px;
        border: none;
        font-size: 24px;
        cursor: pointer;

        /* Custom config */
        padding: 10px 20px;
        border-radius: $--border-radius;
        transition: 0.25s;
        box-sizing: border-box;
        color: $--c1;

        width: 100%;
        text-align: center;
        transition: 0.25s;
    }

    input:hover {
        background-color: $--c2-l5; //#FF8F0F;
    }

    // Post picture
    .post-picture-picker {
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
        margin-top: 5px;
        margin-bottom: 5px;
    }

    .form {
        display: flex;
        gap: 15px;
        flex-direction: column;
    }
    
    .post-picture-picker:hover {
        background-color: $--c0;
        // background-position: left;
    }

    .post-picture-picker > img {
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

    .select-field{
        margin-top: 5px;
        width: 100%;
        border: 2px solid $--c0-d15;
        border-radius: 5px;
        padding: 0.5em;
    }

    .select-field:focus {  // tammik
        border-color: $--c1;
    }

    .row {
      display: flex;
      flex-wrap: wrap;
      gap: 5px;
      margin: 0px;
      padding: 0px;
    }

    .row > * {
      flex-grow: 1;
    }

</style>
