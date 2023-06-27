import {apiFetch} from "$lib/backend.js";

export async function POST({request, params}) {
    await apiFetch(`/api/group/${params.groupID}/invite/${params.userID}`, request)

    return {
        status: 303,
        headers: {
            Location: `/groups/${params.groupID}`
        }
    }
}
