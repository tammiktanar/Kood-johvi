function flat(arr, nr = 1){
    let res = []
    let hasArray = false
    if (nr != 1 ){
        arr.forEach(givenElement => {
           if (Array.isArray(givenElement)){
               hasArray = true
           } 
        });

        if (hasArray){
            res = flat([].concat.apply([], arr), nr - 1)
        } else {
            res = [].concat.apply([], arr)
        }
    } else {
        res = [].concat.apply([], arr)
    }
    return res
}
