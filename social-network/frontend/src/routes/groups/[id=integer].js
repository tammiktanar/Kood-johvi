import {apiFetch} from "$lib/backend"

export async function GET({request, params, locals}) {
    const pGroup = apiFetch(`/api/group/${params.id}`, request)
    const pPosts = apiFetch(`/api/group/${params.id}/posts`, request)
    const pMembers = apiFetch(`/api/group/${params.id}/members`, request)
    const pEvents = apiFetch(`/api/group/${params.id}/events`, request)
    const pFollowing = locals.user ? apiFetch(`/api/user/${locals.user.userID}/following`, request) : Promise.resolve(new Response(JSON.stringify([])));

    const [
        group_response,
        posts_response,
        members_response,
        events_response,
        followings_response
    ] = await Promise.all([
        pGroup,
        pPosts,
        pMembers,
        pEvents,
        pFollowing
    ])



    if (!group_response.ok) {
        return {
            status: group_response.status
        }
    }
    if (!posts_response.ok) {
        return {
            status: posts_response.status
        }
    }

    if (!members_response.ok) {
        return {
            status: members_response.status
        }
    }

    if (!events_response.ok) {
        return {
            status: events_response.status
        }
    }


    if (!followings_response.ok) {
        return {
            status: followings_response.status
        }
    }


    const group = await group_response.json()
    const posts = await posts_response.json()
    const members = await members_response.json()
    const events = await events_response.json()
    const followList = await followings_response.json()

    return {
        body: {
            group: group,
            posts: posts,
            events: events,
            members: members,
            followList: followList
        }
    }
}
