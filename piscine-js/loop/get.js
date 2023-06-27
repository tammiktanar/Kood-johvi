function get(src, path){
    let res = ""
    let curObj = src
    let pathsToGo = path.split(".") 
    for (let i = 0; i < pathsToGo.length; i++) {
        if (curObj === undefined){
            res = undefined
        }else {
            curObj = curObj[pathsToGo[i]]
            if (i == pathsToGo.length-1){
                res = curObj
            }
        }
    }

    return res
}