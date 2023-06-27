function addWeek(givenDate){
    let delta = Math.abs(new Date('0001-01-01') - givenDate) / 1000;
    let days = Math.floor(delta / 86400);
    let res = ""
    switch (days%14) {
        case 0:
            res = "Monday";
            break;
        case 1:
            res = "Tuesday";
            break;
        case 2:
            res = "Wednesday";
            break;
        case 3:
            res = "Thursday";
            break;
        case 4:
            res = "Friday";
            break;
        case 5:
            res = "Saturday";
            break;
        case 6:
            res = "Sunday";
            break;
        case 7:
            res = "secondMonday";
            break;
        case 8:
            res = "secondTuesday";
            break;
        case 9:
            res = "secondWednesday";
            break;
        case 10:
            res = "secondThursday";
            break;
        case 11:
            res = "secondFriday";
            break;
        case 12:
            res = "secondSaturday";
            break;
        case 13:
            res = "secondSunday";
            break;
        default:
            break;
    }
    return res
}

function timeTravel(obj){
    let givenDate = obj.date
    return new Date(givenDate.setHours(obj.hour, obj.minute, obj.second))
}
