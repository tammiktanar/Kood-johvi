function sameAmount(str,reg1,reg2){
    let res = false
    let regex1 = new RegExp(reg1,'g')
    let regex2 = new RegExp(reg2,'g')
    let resRegex1 = str.match(regex1)
    let resRegex2 = str.match(regex2)


    if((resRegex1 != null&&resRegex2!=null)&&(resRegex1.length === resRegex2.length)){
        res = true
    }
    return res
}