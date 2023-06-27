import {apiFetch} from "$lib/backend"

export async function GET({request, params}) {
    const response = await apiFetch(`/api/event/${params.id}`, request)
    if (!response.ok) {
        return {
            status: response.status
        }
    }

    let event = await response.json()

    const group_response = await apiFetch(`/api/group/${event.groupID}`, request)
    if (!group_response.ok) {
        return {
            status: group_response.status
        }
    }

    let group = await group_response.json()


    return {
        body: {
            event: event,
            group: group
        }
    }
}
