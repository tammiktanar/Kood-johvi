<script>
    import {session} from '$app/stores';
    import {goto} from "$app/navigation";

    let cancel = false


    if ($session == null) {
        cancel = true
        goto("/feed")
    }

    let user = $session;
    let user_image = "/img/no-profile-picture.jpg"
    let privateToggle = user.private

    // handler is a function that receives the form's data as  
    if (user.image != null) {
            user_image = "/api/file/"+user.image
    }

    let pictureSelected = false;
    const defaultImage = user_image;
    // For live preview of the image (without uploading)
    function changePreview() {
        let userPictureInput = document.querySelector(
            "#user-picture-input"
        );
        let file = userPictureInput.files[0];
        let imagePreview = document.querySelector("#image-preview");
        let src = file ? URL.createObjectURL(file) : defaultImage;
        imagePreview.src = src;
        pictureSelected = defaultImage != src;
    }
    function deselectUserPicture (e) {
        let userPictureInput = document.querySelector(
            "#user-picture-input"
        );
        userPictureInput.value = null;
        changePreview();
    }




    const intermediate = (e) => {
        const formData = new FormData(e.target)
        const formObject = Object.fromEntries(formData.entries())

        saveEdits(formObject)
        }

        async function saveEdits(formObject) {
            let continueUserEdit = true;

            formObject.password = null;
            formObject.image = null;

            formObject["private"] = privateToggle

            let fileElement = document.querySelector("#user-picture-input");
            if (fileElement.files.length == 1) {
                const imageFormData = new FormData(
                    document.querySelector("#image-form")
                );
                // Filesize check before uploading
                continueUserEdit = fileElement.files[0].size < 1048576;
                if (continueUserEdit) {
                    // Image dimensions check
                    let img = new Image();
                    let imagePreview = document.querySelector("#image-preview");
                    img.src = imagePreview.src;
                    continueUserEdit = img.width == img.height;
                    if (continueUserEdit) {
                        // Upload the file
                        const uploadResponse = await fetch("/api/file", {
                            method: "POST",
                            body: imageFormData,
                        });
                        // Add the received token to the request to be sent to the backend
                        let uploadJsonResponse = await uploadResponse.json();
                        formObject["image"] = uploadJsonResponse.token;
                    } else {
                        alert("File does not fit correct ratio 1:1")
                    }
                } else {
                    alert("File size is too big")
                }
            }

            if (continueUserEdit){

                
                const response = await fetch('/api/user', {
                    method: 'POST',
                    body: JSON.stringify(formObject),
                })


                if (!response.ok) {
                    console.error("Could not save: " + await response.text())
                    return
                }

                const session_response = await fetch('/api/user', {
                    method: 'GET',
                })


                if (!session_response.ok) {
                    console.error("Could not load session data: " + await session_response.text())
                    return
                }

                $session = await session_response.json();

                await goto(`/user/` + $session.userID)
            } else {
                alert("user Image does not fit requirements")
            }
            
        }

</script>

{#if !cancel}
<form on:submit|preventDefault={intermediate}>
    <div class="container-main">
        <div class="container-left">
            <div class="container-image">
                <img src={user_image} alt="Profile Picture">
            </div>
            <div class="image-upload-div">
                <form id="image-form" hidden>
                    <input
                        id="user-picture-input"
                        type="file"
                        accept="image/*"
                        name="file"
                        on:change={changePreview}
                    />
                </form>
                <label
                    class="user-picture-picker"
                    for="user-picture-input"
                >
                    <img src={defaultImage} alt="" id="image-preview" />
                    Choose picture</label
                >
                {#if pictureSelected}
                    <button
                        type="button"
                        class="close-button user-picture-picker"
                        on:click={deselectUserPicture}>X</button
                    >
                {/if}
            </div>

            <div class="follow-div">
                <a href="{'/user/'+$session.userID}">
                    <input class="button follow-btn cancel-button" type="submit" value="Cancel">
                </a>
                <input id="submit-content" class="button follow-btn" type="submit" value="Save">
            </div>
            <!--
            <ul>
                <li><a href="forum.olari.ee">Website</a></li>
                <li><a href="forum.olari.ee">LinkedIn</a></li>
                <li><a href="forum.olari.ee">Github</a></li>
            </ul>
            -->
        </div>
        <div class="container-right">
            <div class="container-header">
                <div class="container-header-left full-width">
                    <h1>{user.firstName} {user.lastName}</h1>
                    <div class="user-div">
                        <table>
                            <tr>
                                <th>First name</th>
                                <td><input class="user-field" placeholder="First name" name="firstName" value="{user.firstName}" required/></td>
                            </tr>
    
                            <tr>
                                <th>Last name</th>
                                <td><input class="user-field" placeholder="Last name" name="lastName" value="{user.lastName}" required/></td>
                            </tr>
    
                            <tr>
                                <th>Nickname</th>
                                <td><input class="user-field" placeholder="Nickname" name="nickname" value="{user.nickname}"/></td>
                            </tr>
    
                            <tr>
                                <th>Email</th>
                                <td><input class="user-field" placeholder="Email" name="email" value="{user.email}" required/></td>
                            </tr>
    
                            <tr>
                                <th>Privacy toggle</th>
                                <td>
                                    <input class="user-checkbox" type="checkbox" name="private" bind:checked={privateToggle}/>
                                </td>
                            </tr>
    
                        </table>
                    </div>
                </div>
            </div>
            <div class="container-info full-width">
                <h3>About {user.firstName} {user.lastName}:</h3>
                <div class="user-div">
                    <textarea class="full-width" placeholder="About me" name="about" value="{user.about}"/>
                </div>
            </div>
        </div>
    </div>
</form>

{/if}
<style lang="scss">

    .full-width {
        width: 100%;
    }

    textarea {
        min-height: 100px;
    }

    .user-div {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
    }

    .follow-div {
        grid-column: 1;
        grid-row: 1;
        width: 100%;
        margin-top: 5px;
    }

    .follow-btn {
        width: 100%;
    }

    .container-main {
        margin: 10px;
        display: flex;
        gap: 20px;
    }

    .container-image {
        border-radius: $--border-radius;
        overflow: hidden;
        border: $--c1 $--border-width solid;
        box-sizing: border-box;
    }

    img {
        display: block;
        object-fit: cover;
        width: 256px;
        height: 256px;
    }

    .container-right {
        display: flex;
        flex-direction: column;
        flex-grow: 1;
        gap: 20px;
    }

    .container-header {
        display: flex;
        justify-content: space-between;
    }

    h1, h2, h3, ul, p {
        margin: 0;
    }

    h1 {
        font-weight: bold;
        font-size: 3rem;
    }

    p {
        font-size: 1.2rem;
    }

    h3 {
        font-size: 1.5rem;
    }

    th {
        text-align: left;
    }


    // Post picture
    .user-picture-picker {
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
    .form {
        display: flex;
        gap: 15px;
        flex-direction: column;
    }
    
    .user-picture-picker:hover {
        background-color: $--c0;
        // background-position: left;
    }
    .user-picture-picker > img {
        height: 48px;
        max-width: 110px;
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

    .image-upload-div {
        width: 260px;
        margin-top: 5px;
        display: flex;
        flex-wrap: wrap;
        gap: 5px;
    }

    .image-upload-div > * {
      flex-grow: 1;
    }




    .user-field {
        font-family: inherit;
		padding: 0.1em;
		box-sizing: border-box;
		border: 2px solid $--c0-d15;
		border-radius: 5px;
		line-height: 1.2;
		overflow: hidden;
        resize: none;
    }

    .user-checkbox{
        height: 25px;
        width: 25px;
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

    .cancel-button {
        margin-bottom: 5px;
    }
</style>
