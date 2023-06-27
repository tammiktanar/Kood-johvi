function superTypeOf(x) {
    if (x instanceof Set) {
        return 'Set'
    }

    if (x instanceof Map) {
        return 'Map'
    }

    if (x === null) {
        return 'null'
    }

    if (Array.isArray(x)) {
        return 'Array'
    }

    let type = typeof x
    if (type === 'undefined') {
        return 'undefined'

    }

    return capitalize(type)
}

function capitalize(str) {
    return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase()
}


function arrToSet(arr){ // -> Set { 1, 2, 3 }
    return new Set(arr)
}
function arrToStr(arr){ // -> '1213'
    return arr.join("")
}
function setToArr(set){ // -> [1, 2, 3]
    return Array.from(set)
}
function setToStr(set){ // -> '123'
    return arrToStr(setToArr(set))
}
function strToArr(str){ // -> ['h', 'e', 'l', 'l', 'o']
    return str.split("")
}
function strToSet(str){ // -> Set { 'h', 'e', 'l', 'o' }
    return arrToSet(strToArr(str))
}
function mapToObj(map){ // -> { a: 1, b: 2, '3': 'c', '4': 'd' }
    return Object.fromEntries(map)
}
function objToArr(obj){ // -> [45, 75, 24]
    return Object.values(obj)
}
function objToMap(obj){ // -> Map { 'x' => 45, 'y' => 75, 'radius' => 24 }
    return new Map(Object.entries(obj));
}
function arrToObj(arr){ // -> { '0': 1, '1': 2, '2': 1, '3': 3 }
    return Object.assign({}, arr)
}
function strToObj(str){ // -> { '0': 'h', '1': 'e', '2': 'l', '3': 'l', '4': 'o' }
    return arrToObj(strToArr(str))
}




