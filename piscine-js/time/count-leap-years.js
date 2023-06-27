function countLeapYears(givenDate){
    let date = new Date(givenDate)
    let res = 0
    for (let i = 0; i <= date.getFullYear(); i++) {
        if (isLeapYear(new Date(i, 0, 0))){
            res++
        }
    }
    return res
}
function isLeapYear(date){
    let year = new Date(date).getFullYear()
    return ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0);
}
