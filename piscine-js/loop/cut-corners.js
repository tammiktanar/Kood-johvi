/*


round (like Math.round)
ceil (like Math.ceil)
floor (like Math.floor)
trunc (like Math.trunc)

*/

function round(nr){
    let res = 0
    if (nr < 0){
        return -round(-nr)
    } else {
        if (nr % 1 >= 0.5){
            res = fuckedParseInt(nr) + 1
        } else {
            res = fuckedParseInt(nr) 
        }
    }
    return res
}

function ceil(nr){
    let res = fuckedParseInt(nr)
    if (nr < 0){
        return -floor(-nr)
    } else {
        if (res == nr){
            return res
        } else {
            return res+1
        }
    }
} 

function floor(nr){
    if (nr < 0){
        return (ceil(nr*-1))*-1
    }else {
        return fuckedParseInt(nr)
    }
}

function trunc(nr){
    return nr-nr%1
}


function fuckedParseInt(nr){
    return trunc(nr)
}