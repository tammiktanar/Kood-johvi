function first(arg){
    var res
    if (typeof arg == 'string'){
        res = arg.charAt(0)
    } else {
        res = arg[0]
    }
    return  res
}
function last(arg){
    var res
    if (typeof arg == 'string'){
        res = arg.charAt(arg.length-1)
    } else {
        res = arg[arg.length-1]
    }
    return  res
}
function kiss(arg){
    const res = [last(arg), first(arg)]
    return res
}