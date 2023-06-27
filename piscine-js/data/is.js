is.num = function(x) {
    return typeof x == 'number';
}
/*

    is.num value is a number
    is.nan value is NaN
    is.str value is a string
    is.bool value is a boolean
    is.undef value is undefined
    is.def value is defined
    is.arr value is an array
    is.obj value is a simple object or null objects
    is.fun value is a function
    is.truthy value is truthy
    is.falsy value is falsy

*/

is.nan = function(x) {
    return Number.isNaN(x);
}

is.str = function(x) {
    return typeof x == 'string';
}

is.bool = function(x) {
    return typeof x == 'boolean';
}

is.undef = function(x) {
    return x === undefined;
}

is.def = function(x) {
    return x !== undefined;
}

is.arr = function(x) {
    return Array.isArray(x);
}

is.obj = function(x) {
    return typeof x === 'object' && !Array.isArray(x) && x !== null;
}

is.fun = function(x){
    return typeof x == 'function';
}

is.truthy = function(x){
    return x
}

is.falsy = function(x){
    return !x
}