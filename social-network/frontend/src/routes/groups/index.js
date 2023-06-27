import {apiFetch} from "$lib/backend"

export async function GET({request}) {
    const response = await apiFetch("/api/group/all", request)
    if (!response.ok) {
        throw new Error(await response.text())
    }
    
    const groups = await response.json()
    let myGroups = groups.slice();


    myGroups = myGroups.filter(x => x.includesMe !== false);




    return {
        body: {
            groups: groups,
            mygroups: myGroups
        }
    }
}