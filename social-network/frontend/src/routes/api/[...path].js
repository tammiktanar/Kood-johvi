/*
* This is a dedicated endpoint that handles requests that fall under /api/*
* I think this might be useful for post requests in .svelte files and such.
* */

import {apiFetch} from "$lib/backend.js";

export async function GET({url, request}) {
    return apiFetch(url.pathname + url.search, request);
}

export async function POST({url, request}) {
    return apiFetch(url.pathname + url.search, request)
}
