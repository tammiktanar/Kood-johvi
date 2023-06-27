function repeat(str, nr){
    let res = ""
    for (let i = 0; i < nr; i++) {
        res +=String(str) 
        
    }

    return String(res)
}




function pyramid(str, nr){
    let res = ""
    for (let i = 0; i <= nr; i++) {
        if (i != 0 && i != 1){
            res += "\n"+repeat(" ", (nr-i)*str.length)+repeat(str, i)+repeat(str, i-1)
        } else {
            if (i != 0){
                res += repeat(" ", (nr-i)*str.length)+repeat(str, i)
            }
        }
    }
    return res
}
