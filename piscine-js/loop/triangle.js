function repeat(str, nr){
    let res = ""
    for (let i = 0; i < nr; i++) {
        res +=String(str) 
        
    }

    return String(res)
}




function triangle(str, nr){
    let res = ""
    for (let i = 0; i <= nr; i++) {
        if (i != 0 && i != 1){
            res += "\n"+repeat(str, i)
        } else {
            if (i != 0){
                res += repeat(str, i)
            }
        }
    }
    return res
}