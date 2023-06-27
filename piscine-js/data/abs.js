function isPositive(nr){
    return 0 < nr
}

function abs(nr){
    if (nr==0){
        return 0
    } else {
        if (isNaN(nr)) {
            return NaN
        } else {
            nr = parseFloat(nr)
            if (typeof nr == "number" || typeof nr == "bigint"){
                if (isPositive(nr)){
                    return nr
                } else {
                    return nr * -1
                }
            }
        }
    }
}