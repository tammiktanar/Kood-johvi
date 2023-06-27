
function capitalize(str){
    return yell(str.charAt(0)) + str.slice(1).toLowerCase()
}

function whisper(str){
    return "*" + str.toLowerCase() + "*"
}

function yell(str){
    return str.toUpperCase()
}

function sentence(arr){
    return arr.join(" ")
}

function words(str){
    return str.split(" ")
}