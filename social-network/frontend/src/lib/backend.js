let BACKEND_ADDRESS = "192.168.0.2"
try {
    await fetch("http://192.168.0.2:8888")
} catch (_) {
    BACKEND_ADDRESS = "localhost"
}

const BACKEND_PORT = "8888"
const BACKEND_URL = `http://${BACKEND_ADDRESS}:${BACKEND_PORT}`

/**
 * Fetch something from the backend API.
 * The purpose of this function is to set the correct host and port for the backend url.
 * The path argument should probably start with "/api/".
 * @param {RequestInfo} path
 * @param {Request} [request] - The request that triggered this fetch. Used to forward cookies to the backend.
 * @param {RequestInit} [override]
 * @returns {Promise<Response>}
 */
export async function apiFetch(path, request, override = {}) {
    console.log(`Fetching "${path}" from backend`)
    override.headers = {...Object.fromEntries(request.headers.entries()), ...override.headers}
    delete(override.headers["connection"])

    //let req =  new Request(request, override)

    return fetch(BACKEND_URL + path, new Request(request, override))
        .catch(e => console.error("fetch error: \n", e))
}
