function deepCopy(obj) {
    let res
    if (typeof obj !== "object" || obj === null || obj instanceof RegExp) {
        return obj
    }
	if (Array.isArray(obj)){
		res = []
	} else {
		res = {}
	}

    for (let key in obj) {
        let value = obj[key];
        res[key] = deepCopy(value);
    }
    return res
};
