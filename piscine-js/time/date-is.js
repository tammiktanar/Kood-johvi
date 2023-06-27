function isValid(date){
    if(isNaN(date)|| date == 0){
        return false
    }else{
        return true
    }
}
function isAfter(fDate, sDate){
    let res = false
    if (isValid(fDate) && isValid(sDate)){
        fDate = new Date(fDate)
        sDate = new Date(sDate)
        if (fDate.getTime() > sDate.getTime()){
            res = true
        }
    }
    return res
}

function isBefore(fDate, sDate){
    let res = false
    if (isValid(fDate) && isValid(sDate)){
        fDate = new Date(fDate)
        sDate = new Date(sDate)
        if (fDate.getTime() < sDate.getTime()){
            res = true
        }
    }
    return res
}

function isFuture(givenDate){
    return isAfter(givenDate, new Date())
}

function isPast(givenDate){
    return isBefore(givenDate, new Date())
}

