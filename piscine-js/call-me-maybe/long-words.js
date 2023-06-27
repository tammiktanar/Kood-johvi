let longWords = (arr) => arr.every((elem) => elem.length > 4);

let oneLongWord = (arr) => arr.some((elem) => elem.length > 9);

let noLongWords = (arr) => arr.every((elem) => elem.length < 7);
