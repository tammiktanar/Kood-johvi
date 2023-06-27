import {apiFetch} from "$lib/backend.js";

export async function GET({request}) {
    const userPromise = apiFetch("/api/user/known", request)
    const groupPromise = apiFetch("/api/group/all", request)

    const [uRes, gRes] = await Promise.all(userPromise, groupPromise)

    if (!uRes.ok || !gRes.ok) {
        throw new Error("Failed to fetch users/groups")
    }

    let users = await uRes.json()
    let groups = await gRes.json()

    return {
        body: {
            users,
            groups
        }
    }
}
