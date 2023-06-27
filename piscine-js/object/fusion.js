function fusion(ob1, ob2) {
    if (getVarType(ob1) === getVarType(ob2)) {
        switch (getVarType(ob1)) {
        case "array":
            return [...ob1, ...ob2];
        case "string":
            return ob1 + " " + ob2;
        case "number":
            return ob1 + ob2;
        case "object":
            const newObj = {
            ...ob1,
            };
            for (let each of Object.keys(ob2)) {
            newObj[each] = fusion(newObj[each], ob2[each]);
            }
            return newObj;
        default:
            return ob2;
        }
    }
    return ob2;
};
function getVarType(data) {
    if (data instanceof Array) {
        return "array";
    }
    if (data instanceof RegExp) {
        return "expression";
    }
    if (data instanceof Set) {
        return "set";
    }
    if (data === null) {
        return "null";
    }
    return typeof data;
};