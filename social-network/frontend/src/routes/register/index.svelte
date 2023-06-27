<script>
    import { goto } from "$app/navigation";
    import { session } from "$app/stores";

    import FormError from "$lib/components/formError.svelte";

    const defaultImage = "/img/no-profile-picture.jpg";

    let pictureSelected = false;

    let errors = {
        passwordMismatch: false,
        emailMismatch: false,
        fileSize: false,
        imageDimensions: false,
        failedToRegister: false
    };

    async function submitForm(e) {
        // Deactivate all errors
        Object.keys(errors).forEach((key) => {
            errors[key] = false;
        });

        const formData = new FormData(e.target);
        const formObject = Object.fromEntries(formData.entries());
        formObject.birthday = new Date(formObject.birthday).toISOString();

        // CLIENT-SIDE VALIDATION
        errors.passwordMismatch =
            formObject.password !== formObject.passwordConfirm;
        errors.emailMismatch = formObject.email !== formObject.emailConfirm;

        // If the <input> contains only a single file
        let fileElement = document.querySelector("#profile-picture-input");

        if (fileElement.files.length == 1) {
            const imageFormData = new FormData(
                document.querySelector("#image-form")
            );

            // Filesize check before uploading
            errors.fileSize = fileElement.files[0].size > 1048576;

            // Image dimensions check
            let img = new Image();
            let imagePreview = document.querySelector("#image-preview");
            img.src = imagePreview.src;
            errors.imageDimensions = img.width != img.height;

            // ##########################
            // Check if we got any errors
            if (
                Object.values(errors).some((v) => {
                    return v;
                })
            ) {
                return;
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

        // ##########################
        // Check if we got any errors
        if (
            Object.values(errors).some((v) => {
                return v;
            })
        ) {
            return;
        }
        // ##########################

        // Actually registering the user
        const response = await fetch("/api/register", {
            method: "POST",
            body: JSON.stringify(formObject),
        });


        if (response.ok) {
            $session = await response.json();
            // Cookie fix
            window.location.href = "/"
            // await goto("/");
        } else {
            errors.failedToRegister = true
        }
    }

    // For live preview of the image (without uploading)
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
    <div class="form-container">
        <div class="heading-container">
            <h1 class="heading-text">REGISTER YOURSELF</h1>
        </div>
        <form class="form" on:submit|preventDefault={submitForm}>
            <div class="row">
                <input
                    class="textinput"
                    name="firstName"
                    type="text"
                    placeholder="First name"
                    required
                />
                <input
                    class="textinput"
                    name="lastName"
                    type="text"
                    placeholder="Last name"
                    required
                />
            </div>

            <div class="row">
                <input
                    class="textinput"
                    name="email"
                    type="email"
                    placeholder="E-mail"
                    required
                />
                <input
                    class="textinput"
                    name="emailConfirm"
                    type="email"
                    placeholder="Confirm e-mail"
                    required
                />
            </div>

            <div class="row">
                <input
                    class="textinput"
                    name="password"
                    type="password"
                    placeholder="Password"
                    required
                />
                <input
                    class="textinput"
                    name="passwordConfirm"
                    type="password"
                    placeholder="Confirm password"
                    required
                />
            </div>

            <div class="row">
                <input
                    class="textinput"
                    name="nickname"
                    type="text"
                    placeholder="nickname (optional)"
                />
            </div>
            <div class="row">
                <input
                    class="textinput"
                    name="birthday"
                    type="date"
                    placeholder="Date of birth"
                    required
                />
            </div>

            <div class="row">
                <textarea
                    class="textinput"
                    name="about"
                    placeholder="About me (optional)"
                />
            </div>

            <div class="row">
                <form id="image-form" hidden>
                    <input
                        id="profile-picture-input"
                        type="file"
                        accept="image/*"
                        name="file"
                        on:change={changePreview}
                    />
                </form>
                <label
                    class="profile-picture-picker"
                    for="profile-picture-input"
                >
                    <img src={defaultImage} alt="" id="image-preview" />
                    Choose picture</label
                >
                {#if pictureSelected}
                    <button
                        type="button"
                        class="close-button profile-picture-picker"
                        on:click={deselectProfilePicture}>X</button
                    >
                {/if}
            </div>

            <!--<input name="image" accept="image/png, image/jpeg" type="file"/>-->
            <button class="button" type="submit">Register</button>
        </form>
    </div>

    <!-- Create error component here -->
    {#if errors.emailMismatch}
        <FormError error="Emails don't match" />
    {/if}
    {#if errors.passwordMismatch}
        <FormError error="Passwords don't match" />
    {/if}
    {#if errors.fileSize}
        <FormError error="Profile picture must be under 1 megabyte" />
    {/if}
    {#if errors.imageDimensions}
        <FormError error="Only images with equal with and height" />
    {/if}
    {#if errors.failedToRegister}
        <FormError error="Failed to register account" />
    {/if}
</div>

<style lang="scss">
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

    .form {
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
    }

    input:focus { // tammik
        border-color: $--c1;
    }

    textarea:focus { // tammik
        border-color: $--c1;
    }
</style>
