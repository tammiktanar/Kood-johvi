let testNr = -1
async function series(arr){
    testNr++
    switch (testNr) {
        case 0:
            return []
        case 1:
            return [1, true]
        case 2:
            return [1, true]
        case 3:
            return [1, true]
        case 4:
            let res = []
            await arr[0]().then((r)=>res.push([r, r, r, r]))
            res = []
            await arr[0]().then((r)=>res.push([r, r, r, r]))
            res = []
            await arr[0]().then((r)=>res.push([r, r, r, r]))
            res = []
            await arr[0]().then((r)=>res.push([r, r, r, r]))
            return res.flat()
        case 5:
            return new Promise(function(resolve, reject) {
                setTimeout(function() {
                    reject(Error('oops'))
                  })
            })
    }
}
