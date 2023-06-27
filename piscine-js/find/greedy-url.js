/*
Instructions
Write 3 functions :

getURL that returns an array with all valid URLs present in a data-set, http and https
greedyQuery that returns URL with at least 3 or more parameters from all URLs that contain a query.
notSoGreedy that returns URL with at least 2, but not more then 3 parameters from all URLs that contain a query.
You can search for greedy quantifiers for help
*/

function getURL(data) {
    var result = /(https?:\/\/[a-zA-Z0-9]+[^\s]{2,})/g;
    return data.match(result);
}

function greedyQuery(data) {
    var get = getURL(data);
    var arr = [];
    var result = /([^=]*[=]){3,}/g;

    for (let item of get) {
        if (item.match(result) !== null) {
          arr.push(item);
      }
    }
      return arr;
    }

function notSoGreedy(data) {
        let get = getURL(data);
        let arr = [];
        var result = /[=]/g;
        for (let item of get) {
          if ( item.match(result) !== null &&
            item.match(result).length >= 2 &&
            item.match(result).length <= 3)
            {
            arr.push(item);
          }
        }
        return arr;

}