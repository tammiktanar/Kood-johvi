function bloodySunday(givenDate){
    let delta = Math.abs(new Date('0001-01-01') - givenDate) / 1000;
    let days = Math.floor(delta / 86400);
    let res = ""
    switch (days%6) {
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
        default:
            break;
    }
    return res
}
