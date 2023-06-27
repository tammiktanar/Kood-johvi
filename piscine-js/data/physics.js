function isNumber(nr){
    return typeof nr === 'bigint' || typeof nr === 'number';
}

function getAcceleration(obj){
    let res = "impossible"
   
    if (isNumber(obj.f) && isNumber(obj.m)){
        res = obj.f/obj.m
    } else if (isNumber(obj.Δv) && isNumber(obj.Δt)){
        res = obj.Δv/obj.Δt
    } else if (isNumber(obj.d) && isNumber(obj.t)){
        res = (2 * obj.d)/(obj.t*obj.t)
    } else {
        res = "impossible"
    }




    return res
}