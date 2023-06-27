function chunk(arr, nr){
    let res = []
    let tempResComponent = []
    let curNr = 0
    for (let i = 0; i < arr.length; i++) {
        if (curNr == nr){
            res.push(tempResComponent)
            tempResComponent = []
            curNr = 0
        }

        tempResComponent.push(arr[i])
        curNr++
                
        if (i == arr.length-1){
            res.push(tempResComponent)
        }
    }
    return res
}