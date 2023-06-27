function slice(arr, start=0, end = arr.length){
    let res = []
    let isString = false

    if (start < 0) {
        start = arr.length+start
    }

    if (end < 0) {
        end = arr.length+end
    }

    if (!Array.isArray(arr)) {
        isString = true
        arr = arr.split("");
    }

    for (let i = start; i < end; i++) {
        res.push(arr[i]);
    }


    if (isString){
        res = res.join("")
    }

    return res
}
