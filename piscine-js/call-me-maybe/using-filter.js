let filterShortStateName = (arr) => arr.filter((elem) => elem.length < 7);

let filterStartVowel = (arr) =>
  arr.filter((elem) => /[a|i|u|e|o]/gi.test(elem[0]));

let filter5Vowels = (arr) =>
  arr.filter((elem) => elem.match(/[a|i|u|e|o]/gi).length > 4);

let filter1DistinctVowel = (arr) =>
  arr.filter(function (elem) {
    let chs = (str) => str.match(new RegExp(str[0], "ig")).length == str.length;
    return chs(elem.match(/[i|a|u|e|o]/gi).join(""));
  });
  
let multiFilter = (arrObj) =>
  arrObj.filter(
    (elem) =>
      elem["capital"].length > 7 &&
      !/[a|i|u|e|o]/gi.test(elem["name"][0]) &&
      /[a|i|u|e|o]/gi.test(elem["tag"]) &&
      elem["region"] !== "South"
  );
