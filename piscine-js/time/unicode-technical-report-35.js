function format(givenDate, formatGiven){
    let res = String(formatGiven);
    let date = new Date(givenDate);

        

    if (res.includes("yyyy")){
        res = res.replace(/yyyy/g, Math.abs(date.getFullYear()).toString().padStart(4, "0"))
    } else {
        res = res.replace(/y/g, Math.abs(date.getFullYear()))
    }


    if (res.includes("dd")){
        res = res.replace(/dd/g, date.getDate().toString().padStart(2, "0"))
    }else {
        res = res.replace(/d/g, date.getDate().toString())
    }
    if (res.includes("hh")){
        res = res.replace(/hh/g, String(H(date)).padStart(2, "0"))
    }else{
        res = res.replace(/h/g, H(date))
    }

    if (res.includes("mm")){
        res = res.replace(/mm/g, date.getMinutes().toString().padStart(2, "0"))
    }else {
        res = res.replace(/m/g, date.getMinutes())
    }

    if (res.includes("MMMM")){
        res = res.replace(/MMMM/g, monthsLong[date.getMonth()])
    } else if (res.includes("MMM")){
         res = res.replace(/MMM/g, months[date.getMonth()])
    } else if (res.includes("MM")){
         res = res.replace(/MM/g, (date.getMonth()+1).toString().padStart(2, "0"))
    } else {
        res = res.replace(/M/g, (date.getMonth()+1).toString())
    }

    if (res.includes("EEEE")){
        res = res.replace(/EEEE/g, daysLong[date.getDay()])
    } else {
        res = res.replace(/E/g, days[date.getDay()])
    }

    if (formatGiven.includes("a")){
        res =  res.replace(/a/g, a(date))
    }
    res = res.replace(/ss/g, date.getSeconds().toString().padStart(2, "0"))
    res = res.replace(/s/g, date.getSeconds())
    
    res = res.replace(/HH/g, date.getHours().toString().padStart(2, "0"))
    res = res.replace(/H/g, date.getHours())

    res = res.replace(/GGGG/g, gggg(date))
    res = res.replace(/G/g, g(date))
    

    return res
}

const months = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
]

const monthsLong = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
]

const days = [
    'Sun',
    'Mon',
    'Tue',
    'Wed',
    'Thu',
    'Fri',
    'Sat',
]

const daysLong = [
    'Sunday',
    'Monday',
    'Tuesday',
    'Wednesday',
    'Thursday',
    'Friday',
    'Saturday',
]


function H(date){
    var hours = date.getHours();
    return hours%12
}

function a(date){
    var hours = date.getHours();
    var ampm = hours >= 12 ? 'PM' : 'AM';
    return ampm
}




function gggg(date){
    let res = ""
    if (date.getFullYear() > 0){
        res = "Anno Domini"
    } else {
        res = "Before Christ"
    }
    return res
}

function g(date){
    let res = ""
    if (date.getFullYear() > 0){
        res = "AD"
    }else {
        res = "BC"
    }
    return res
}












