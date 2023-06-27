import { loadProfile } from "../graphql.js"
import { fetchDataWithQuery } from "./query.js"

async function getListOfUsers(offset = 0, users = []){
    let query = `{
        user (
          offset: ${offset},
          limit: 20
        ){
          value: id
          label: login
        }
    }`

    let returned_users = (await fetchDataWithQuery(query)).data.user

    if (returned_users.length > 0) {
        users = users.concat(returned_users)
        return (await getListOfUsers(offset+20, users))
    }

    return users
}



export async function createUserSearch(){
    let usernames = await getListOfUsers()
    let srchField = document.getElementById('srch-user')
    let search = new Autocomplete(srchField, {
        data: usernames,

        highlightTyped: true,
        highlightClass: 'text-primary',
        onSelectItem: ({label, value}) => {
            console.log("user selected:", label, value);
            if (value != null && value != undefined){
                loadProfile(value)
            }
        }
    })
    srchField.removeAttribute("disabled");
}

