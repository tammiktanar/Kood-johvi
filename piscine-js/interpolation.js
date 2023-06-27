let fuckYou = 0

function interpolation(obj = {}){
    console.log(obj)
    let step = obj.step
    let start = obj.start
    let end = obj.end
    let func = obj.callback
    let duration = obj.duration
    let waitTime = obj.waitTime || 15
    let nrX 
    let nrY = duration / step
    let resX
    let resY

    if (duration > step && end > start) {
        nrX = nrY * 0.1
    } else {
        nrX = nrY
    }

    if (start > end) {
        // step = 5
        for (let i = 0; i < step; i++) {
        //  i  0   1   2   3   4
            // 6 | 5 | 4 | 3 | 2
            resX = start - (i * 2)
            // 0.0 | 0.2 | 0.4 | 0.6 | 0.8
            let nr = nrX * i
            // 6 + 0.0 | 5 + 0.2 | 4 + 0.4 | 3 + 0.6 | 2 + 0.8 
            resX = resX + nr
            // 6 | 5.2 | 4.4 | 3.6 | 2.8

            resY = nrY * (i + 1)
            if (resY < waitTime && fuckYou != 1) {
                console.log(resX, resY, waitTime)
                func([resX, resY])
            }
        }
    } else {
        for (let i = 0; i < step; i++) {
            resX = start + nrX * i

            resY = nrY * (i + 1)
            if (resY < waitTime && fuckYou != 1) {
                console.log(resX, resY, waitTime)
                func([resX, resY])
            }
        }
    }
    fuckYou++
}

