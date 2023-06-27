function indexOf(arr, str, fromIndex = 0){
    let res = -1
    for (let i = fromIndex; i < arr.length; i++) {
        if (arr[i] == str && res == -1){
            res = i
        }
    }
    return res
}

function lastIndexOf(arr, str, fromIndex = arr.length){ 
    let res = -1
    for (let i = fromIndex; i >= 0; i--) {
        if (arr[i] == str && res == -1){
            res = i
        }
    }

    return res
}

function includes(arr, str){
    let res = false

    for (let i = 0; i < arr.length; i++) {
        if (arr[i] == str){
            res = true
        }
    }

    return res
}