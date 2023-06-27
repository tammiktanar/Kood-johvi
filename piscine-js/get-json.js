async function getJSON(path, params = {}){
    let getURL = path 
    let keys = Object.keys(params)
    let res

    keys.forEach((key, i) => {
        if (i == 0){
            getURL += "?"+key+"="+String(params[key]).trim().replace(" ", "+")
        } else {
            getURL += "&"+key+"="+String(params[key]).trim().replace(" ", "+")
        }
    });

    res = await fetch(getURL)
    .then(response => response.json())
    .then(data => {return data})
    .catch((error) => {
        throw error
    });


    if (await res.error){
        throw Error(res.error)
    } else if ( await res.data){
        return res.data
    } else if (await res.status != 200){
        throw Error("Internal Server Error")
    }
}