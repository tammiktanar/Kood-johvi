/** @type {import('./__types/[id]').RequestHandler} */

import { apiFetch } from "$lib/backend";

import {User, userGet} from "../../lib/services/user";

export async function GET({ params, request, locals }) {
    let posts = []
    let followers = []
    let followings = []

    // Try converting the string ID to number type
    let userId = parseInt(params.id)

    let errorMsg
    let user = await userGet(request, userId).catch(reason => {
        errorMsg = reason
    })

    if (!userId) {
        if (locals.user != null) {

            user = locals.user
            user.image = locals.user.image
            user.access = true

        }
    }

    if (user) { // If user exists
        const post_response = await apiFetch(`/api/user/${user.userID}/posts`, request)
        const followers_response = await apiFetch(`/api/user/${user.userID}/followers`, request)
        const followings_response = await apiFetch(`/api/user/${user.userID}/following`, request)


        if (post_response.ok) { // Get posts
            posts = await post_response.json()
        } 

        if (followers_response.ok) { // Get posts
            followers = await followers_response.json()
        } 

        if (followings_response.ok) { // Get posts
            followings = await followings_response.json()
        } 


        return {
            status: 200,
            body: { 
                user: User(user),
                posts: posts,
                followers: followers,
                followings: followings,
                access: user.access
            }
        }
    }

    return {
        status: 404,
        body: new Error(errorMsg)
    }

}
