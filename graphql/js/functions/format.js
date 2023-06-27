export function getXPFormatStr(xp){
    let type = ""
    let nr = xp

    if (!(nr < 0)) {
        if (nr < 1000) {
            type = 'B'
        }

        if (nr >= 1000){
            type = 'kB'
            nr = (nr / 1000)
        }

        if (nr >= 1000) {
            type = 'mB'
            nr = (nr / 1000)
        }
        
    }

    return (Math.round(nr*100)/100) + type
}

export function getDateFormat(givenDate){
    let date = new Date(givenDate)
    let res = date.getDate() + "/" + (date.getMonth()+1)+ "/" + date.getFullYear()
    return res
}


export function getLevel(xp) {
    let level = 0

    while (levelNeededXP(++level) < xp) {}

    return level-1
}

export function levelNeededXP(level) {
    return Math.round(level * (176 + 3 * level * (47 + 11 * level)))
}
