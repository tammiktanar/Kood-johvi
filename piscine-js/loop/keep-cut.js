/*
Create the cutFirst function that takes a string and remove the 2 first characters.

Create the cutLast function that takes a string and remove the 2 last characters.

Create the cutFirstLast function that takes a string as parameter and remove the 2 first characters and 2 last characters.

Create a keepFirst function that takes a string as parameter and return the string only keeping the 2 first characters.

Create a keepLast function that takes a string as parameter and return the string only keeping the 2 last characters.

Create a keepFirstLast function that takes a string as parameter and only keep 2 first characters and 2 last characters.

*/

function cutFirst(str){
    return str.slice(2, str.length)
}

function cutLast(str){
    return str.slice(0, str.length-2)
}

function cutFirstLast(str){
    return cutLast(cutFirst(str))
}

function keepFirst(str){
    return str.slice(0, 2)
}

function keepLast(str){
    return str.slice(str.length-2)
}

function keepFirstLast(str){
    if (str.length >=4){
        return keepFirst(str)+keepLast(str)
    } else {
        return str
    }
}

console.log(keepFirstLast('af') )