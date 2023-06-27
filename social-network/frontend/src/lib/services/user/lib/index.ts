import type { UserGetType } from "../type/UserType"
import { apiFetch } from "$lib/backend.js"

const USER_GET_PATH = (userId?: number): string =>
    `/api/user/${!!userId ? userId : ""}`

export const userGet = async (request: Request, id?: number): Promise<UserGetType> => {
    const response = await apiFetch(USER_GET_PATH(id), request, { method: "GET" })

    if (response.status != 200) {
        return Promise.reject(`User with ID '${id}' does not exist`)
    }

    return response.json()
}
