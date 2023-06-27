function clone(obj) {
    if (null == obj || "object" != typeof obj) return obj;
    var copy = obj.constructor();
    for (var attr in obj) {
        if (obj.hasOwnProperty(attr)) copy[attr] = obj[attr];
    }
    return copy;
}

const clone1  = clone(person)
const clone2  = clone(person)
const samePerson  = person

person.age++
person.country = "FR"


