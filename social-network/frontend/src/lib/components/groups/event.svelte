<script>
    import { invalidate } from "$app/navigation";


    export let includesMe = false
    export let event = {
        eventID: 0,
        groupID: 0,
        title: "",
        about: "",
        time: "",
        myStatus: ""
    }

    const intermediateNotGoing = (e) => {
        const formData = new FormData(e.target)
        const formObject = Object.fromEntries(formData.entries())
        changeGoingState("dont")
    }

    const intermediateGoing = (e) => {
        const formData = new FormData(e.target)
        const formObject = Object.fromEntries(formData.entries())
        changeGoingState("go")
    }

    async function changeGoingState(goingState){
        let url = ``

        switch (goingState) {
            case 'go':
                url = `/api/event/${event.eventID}/going`
                break;
            case 'dont':
                url = `/api/event/${event.eventID}/not-going`
                break;
            default:
                break;
        }


        const response =  await fetch(url, {
            method: 'POST',
            body: ""
        })

        if (!response.ok) {
            console.error("Could not attend event: " + await response.text())
            return
        }

        invalidate(`/event/${event.eventID}`)
        invalidate(`/groups/${event.groupID}`)
    }

</script>

<div class="event-div">
    <address><a class="event-name" href="/event/{event.eventID}">{event.title}</address>
    {new Date(event.time).toLocaleString()}
    {#if includesMe}
        <div class="event-state">
            {#if event.myStatus === "UNSET"}
            <form on:submit|preventDefault={intermediateNotGoing}>
                <input  class="button event-button" type="submit" value="Don't go to event">
            </form>
            <form on:submit|preventDefault={intermediateGoing}>
                <input  class="button event-button" type="submit" value="Go to event">
            </form>
            {:else if event.myStatus === "GOING"}
                <form on:submit|preventDefault={intermediateNotGoing}>
                    <input  class="button event-button" type="submit" value="Don't go to event">
                </form>

            {:else if event.myStatus === "NOT_GOING"}
                <form on:submit|preventDefault={intermediateGoing}>
                    <input  class="button event-button" type="submit" value="Go to event">
                </form>
            {/if}
        </div>
    {/if}
</div>


<style lang="scss">
    .event-div {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
        display: flex;
        justify-content: space-between;
        align-items: baseline;
    }

    .event-state {
        display: flex;
        gap: 5px;
    }



    .event-name {
        text-decoration: none;
    }

    .event-name:hover{
        color: $--c2-d5;
        cursor: pointer;
    }
    
</style>
