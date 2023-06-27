function findExpression(nr, curNr = 1, path = "1"){
    let res = "1"
    if (curNr == nr) {
        res = path
    } else {
        if (curNr < nr){
            res = findExpression(nr, curNr+4, (path+" "+add4))
            if (res == undefined){
                res = findExpression(nr, curNr*2, (path+" "+mul2))
                if (res == undefined){
                    res = undefined
                }
            }
        } else {
            res = undefined
        }
    }

    return res
}