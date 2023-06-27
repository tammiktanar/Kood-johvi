<script>
    import { goto } from "$app/navigation";
	import Event from "$lib/components/groups/event.svelte"
	
    export let event
    export let group
    
    function openGroup(groupID){
        goto(`/groups/${groupID}`)
    }

</script>

    <input on:click={openGroup(event.groupID)} class="button open-group-button " type="submit" value="Open group">

<Event event={event} includesMe={group.includesMe}/>

<div class="event-about">
    <p>{event.about}</p>
</div>

<style lang="scss">
    .open-group-button {
        margin-left: 0.4em;
    }

    .event-about {
        background-color: $--c0-l10;
        border-radius: $--border-radius;
        padding: 0.5em 1em;
        margin: 5px 10px;
    }

</style>
