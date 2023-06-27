function isFriday(date){
    date = new Date(date)
    return date.getDay()==5
}

function isWeekend(date){
    date = new Date(date)
    return date.getDay() == 0 || date.getDay() == 6    
}
function isLeapYear(date){
    let year = new Date(date).getFullYear()
    return ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0);
}

function isLastDayOfMonth(date){
    date = new Date(date);
    return date.getMonth() != addDays(date, 1).getMonth()
}

function addDays(date, days) {
    let result = new Date(date);
    result.setDate(result.getDate() + days);
    return result;
}