function invert(obj){
    let res = {}
    let keys = Object.keys(obj)
    for (let i = 0; i < keys.length; i++) {
        res[obj[keys[i]]] = keys[i] 
        
    }


    return res
}