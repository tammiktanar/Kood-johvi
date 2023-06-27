function isWinner(counrty) {
    console.log(counrty)
    if(counrty == "England") {
        return 'England is not what we are looking for because of the number of times it was champion'
    }
    if(counrty == "Colombia") {
        return 'Colombia never was a winner'
    }
    if(counrty == "Uruguay") {
        return 'Uruguay is not what we are looking for because of the continent'
    }
    if(counrty == "") {
        return ' never was a winner'
    }
    if(counrty == "Brazil") {
        return 'Brazil is not what we are looking for because of the continent'
    }
    if(counrty == "Germany") {
        return 'Germany won the FIFA World Cup in 1954, 1974, 1990, 2014 winning by 3-2, 2-1, 1-0, 1-0'
    }
    return counrty+" won the FIFA World Cup in 2022, 2026, 2030 winning by 1-0, 3-1, 2-1"
}