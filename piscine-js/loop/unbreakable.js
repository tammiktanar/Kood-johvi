
function split(str, breaker){
    let res = []
    let lastBreakerIndex = 0
    let curIndexOf = 0
    let nextIndexOf = 0
    let keepGoing = true
    while (keepGoing) {
        curIndexOf = str.indexOf(breaker, lastBreakerIndex) + 1

        if (curIndexOf != -1){
            res.push( str.slice(lastBreakerIndex, curIndexOf-1) )
            lastBreakerIndex = curIndexOf-1+breaker.length
            nextIndexOf = str.indexOf(breaker, curIndexOf)
            console.log(nextIndexOf)
            if (nextIndexOf == -1){
                res.push( str.slice(curIndexOf-1+breaker.length))
                keepGoing = false
            }
        } else {
            keepGoing = false
        }
    }

    return res
}
console.log(split('ggg - ddd - b', ' - '))

function join(arr, combiner) {
    let res = ""
    for (let i = 0; i < arr.length; i++) {
        if (i != 0){
            res += combiner
        }
        res += arr[i];
    }

    return res
}