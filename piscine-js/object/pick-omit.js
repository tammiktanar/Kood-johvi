function pick(obj, keys){
    let res = {}
    if (typeof keys === 'string'){
        keys = [keys]
    }
    for (let i = 0; i < keys.length; i++) {
        if (obj[keys[i]] != undefined){
            res[keys[i]] = obj[keys[i]]
        }
    }
    return res
}


function omit(obj, keys){
    let allKeys = Object.keys(obj)
    let goodKeys = []

    for (let i = 0; i < allKeys.length; i++) {
        if (!keys.includes(allKeys[i])){
            goodKeys.push(allKeys[i])
        }
    }

    return pick(obj, goodKeys)
}