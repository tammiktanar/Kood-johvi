import {apiFetch} from "$lib/backend.js"

export async function handle({event, resolve}) {
    // If dealing with a GET request to anything other than the API
    if (event.request.method === "GET" && !event.url.pathname.startsWith('/api')) {
        // Set user information
        event.locals.user = await getUserInformation(event.request)
    }

    return resolve(event)
}

async function getUserInformation(request) {
    try {
        const token = getSessionToken(request.headers.get('Cookie'))

        if (!token) return
        const res = await apiFetch("/api/user", request, {method: "GET"})


        if (!res.ok) return
        return await res.json()
        
    } catch (e) {
        console.error(e)
    }
}

const cookieRegex = /(?<=session=)[^;]+/
function getSessionToken(cookieString) {
    return cookieString?.match(cookieRegex)
}

export function getSession({locals}) {
    return locals.user || null;
}
