function multiply(a,b){
    let res = 0
    let nrA = Math.abs(a)
    let nrB = Math.abs(b)
    let isNegative = false
    if ((a<0 || b<0) && ((a<0) != (b<0))){
        isNegative = true
    }

    for (let i = 0; i < nrB; i++) {
        res += nrA
    }

    if (isNegative){
        res = res - res - res
        return res
    } else {
        return res
    }
}

function divide(a,b){
    let res = 0
    let nrA = Math.abs(a)
    let nrB = Math.abs(b)
    let isNegative = false
    if ((a<0 || b<0) && ((a<0) != (b<0))){
        isNegative = true
    }

    while (nrB<nrA) {
        res++
        nrA = nrA-nrB
    }

    if (isNegative){
        res = res - res - res
        return res
    } else {
        return res
    }
}


function modulo(a,b){
    let res = 0
    let nrA = Math.abs(a)
    let nrB = Math.abs(b)
    let isNegative = false
    if (a<0){
        isNegative = true
    }

    while (nrB<nrA) {
        nrA = nrA-nrB
    }
    
    res = nrA

    if (isNegative){
        res = res - res - res
        return res
    } else {
        return res
    }
}