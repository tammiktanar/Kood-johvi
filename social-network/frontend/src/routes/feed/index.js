import {apiFetch} from "$lib/backend"

export async function GET({request}) {
    const response = await apiFetch("/api/post/all", request)
    if (!response.ok) {
        throw new Error(await response.text())
    }
    const posts = await response.json()

    return {
        body: {
            posts
        }
    }
}
