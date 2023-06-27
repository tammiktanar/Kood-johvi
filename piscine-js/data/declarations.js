const escapeStr = "`\\/\"'";
const arr = [4, '2'] 
const obj = {str : "string", num : 2, bool : true, undef : undefined }
const nested = {
    'arr' : [
        4, undefined, '2'
    ],
    'obj' : {
        'str' : obj.str,
        'num' : obj.num,
        'bool' : obj.bool
    }
}

Object.freeze(nested);
for(let key in nested) {
   if(nested.hasOwnProperty(key) && typeof nested[key] === 'object') {
      Object.freeze(nested[key]);
   }
}
