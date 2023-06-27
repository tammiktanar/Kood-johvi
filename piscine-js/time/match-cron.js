function matchCron(cronStr, givenDate){
    let cron = cronStr.split(" ")
    let res = true
    let cronTime = new Date(givenDate)
    if (cron[0] != "*" && cronTime.getMinutes() != Number(cron[0])){
        res = false
    }
    if (cron[1] != "*" && cronTime.getHours() != Number(cron[1])){
        res = false
    }
    if (cron[2] != "*" && cronTime.getDate() != Number(cron[2])){
        res = false
    }
    if (cron[3] != "*" && cronTime.getMonth()+1 != Number(cron[3])){
        res = false
    }
    if (cron[4] != "*" && cronTime.getDay() != Number(cron[4])&7){
        res = false
    }

    return res
}