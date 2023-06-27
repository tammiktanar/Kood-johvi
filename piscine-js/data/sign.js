function sign(nr){  
    if (nr == 0) {
        return 0
    }

    if (nr < 0){
        return -1
    } else {
        return 1
    }
}



function sameSign(nr1, nr2){
    return sign(nr1) == sign(nr2)
}