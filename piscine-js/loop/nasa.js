function nasa(nr){
    let res = ""

    for (let i = 1; i <= nr; i++) {
        let add = ""

        if (i%3 == 0){
            add += "NA"
        }

        if (i%5 == 0){
            add += "SA"
        }
        
        if (add == "") {
            add = String(i)
        }

        if (i != 1){
            res +=  " "
        }
        res += add
    }

    return res
}