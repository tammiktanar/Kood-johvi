<script>
    import List from "$lib/components/selectionlist/list.svelte"
    import BasicButton from "$lib/components/buttons/basicButton.svelte"

    let selection = "users"

    let names = {
        users: [
            "Kanguste",
            "Kris",
            "Olaroll",
            "Tammiktanar",
            "Urist",
            "AaEnnDeeErrEeEss",
            "HEEEEEEEELOOOOOOOOOOOOOOO"
        ],
        groups: [
            "Anxierty",
            "Depression",
            "Programming news",
            "Johnny Depp",
            "Ukraine",
            "Covid-19",
            "Kood/Jõhvi"
        ]
    }
</script>

<div class="container">
    <div class="buttons">
        <input type="radio" name="selection" bind:group={selection} value={"users"} id="users">
        <label for="users">
            Users
        </label>
        
        <input type="radio" name="selection" bind:group={selection} value={"groups"} id="groups">
        <label for="groups">
            Groups
        </label>
    </div>
    <List labels={names[selection]} />
</div>

<style lang="scss">
    .container {
        display: flex;
        flex-direction: column;
        gap: 10px;
        width: 250px;
    }

    .buttons {
        display: flex;
        justify-content: space-evenly;
        gap: 5px;
    }

    input {
        opacity: 0;
        position: fixed;
        width: 0;
    }

    input:checked + label {
        background-color: $--c2-l15;
    }

    label {
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

    label:hover {
        background-color: $--c2-l20;
    }
</style>