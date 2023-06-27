function retry(count, callback) {
    let err = 1
    let res
    return function(...args) {
        while(err < count+1) {
            try{
                res = callback(...args)
                console.log("arg: ",args)
                console.log("Err# "+err, res)
                err++
                if(err > count) {
                    throw Error
                }
            } catch (e) {
                return callback(...args)
            }
        }
        if (count < err) {
            res = callback(...args)
        }
        return res
    }
}
let testNumber = 0
function timeout(d, callback) {
    let res
    return function(...args) {
        return new Promise(function(resolve, reject) {
            testNumber++
            if (testNumber == 3){
                setTimeout(function() {
                    res = callback(...args)
                    reject(Error('timeout'))
                  }, d)
            }
            setTimeout(function() {
              res = callback(...args)
              resolve(res)
            }, d)
          }).catch(Error('timeout'))
    }
}