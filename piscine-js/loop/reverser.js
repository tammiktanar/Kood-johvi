function reverse(arr){
    let res = []
    let isString = false
    if (!Array.isArray(arr)) {
        isString = true
        arr = arr.split("");
    }

    for (let i = arr.length-1; i >= 0; i--) {
        res.push(arr[i])
        
    }


    if (isString){
        res = res.join("")
    }

    return res
}