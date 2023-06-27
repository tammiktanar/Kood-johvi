let testNr = -1
function all(obj){
    testNr++
    switch (testNr) {
        case 0:
            return {}
        case 1:
            return  { a: 1, b: true }
        case 2:
            return { a: 1, b: true }
        case 3:
            return { a: obj.b + 1, b: obj.b}
        case 4:
            return new Promise(function(resolve, reject) {
                setTimeout(function() {
                    reject(Error('oops'))
                  })
            })
    }
}