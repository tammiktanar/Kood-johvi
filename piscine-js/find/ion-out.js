function ionOut(str){
    let regEx = /(\w*)(?=tion)/g
    let resReg = str.match(regEx)
    let res = []
    if(resReg === null){
        return res
    }
    for(let i=0;i<resReg.length;i++){
        if(i%2 === 0){
            resReg[i] = resReg[i] + 't'
            res.push(resReg[i])
        }
    }
    return res
}