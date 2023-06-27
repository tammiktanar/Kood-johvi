import {apiFetch} from "$lib/backend"

export async function GET({request, params}) {
    const pPost =  apiFetch(`/api/post/${params.id}`, request)
    const pComments = apiFetch(`/api/post/${params.id}/comment/all`, request)

    const [response, commentResponse] = await Promise.all([pPost, pComments])

    if (!response.ok) {
        return {
            status: response.status
        }
    }

    if (!commentResponse.ok) {
        return {
            status: commentResponse.status
        }
    }

    let post = await response.json()
    const comments = await commentResponse.json()
    
    post["comments"] = comments

    return {
        body: {
            post
        }
    }
}
