export async function fetchDataWithQuery(givenQuery, givenVariables = {}){
    let query = JSON.stringify({
        query: givenQuery,
        variables: givenVariables
    })

    const options = {
        method: "post",
        headers: {
          "Content-Type": "application/json"
        },
        body: query
    }
    
    let res = await fetch(`https://01.kood.tech/api/graphql-engine/v1/graphql`, options)
    
    return res.json()
}