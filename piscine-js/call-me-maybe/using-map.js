function citiesOnly(arr) {
    return arr.map(function (elem) {
        return elem["city"];
    });
}
function upperCasingStates(arr){
    return arr.map(function (string) {
        let arr = string.split(" ");
        return arr.map(function (string) {
            return string.charAt(0).toUpperCase() + string.slice(1);
        }).join(" ");
    });
}

function fahrenheitToCelsius(arr){
    return arr.map(function (str) {
        return Math.floor(((str.split("°F")[0] - 32) * 5) / 9) + "°C";
    });
}

function trimTemp(arr){
    return arr.map(function (obj){
        obj["temperature"] = obj["temperature"].match(/\S/g).join("")
        return obj
    });
}

function trimTemp2(arr) {
    return arr.map(function (obj) {
      return obj["temperature"].match(/\S/g).join("");
    });
}
function tempForecasts(arr) {
    return arr.map(function (obj) {
        return (
            fahrenheitToCelsius(trimTemp2([obj])) + "elsius in " + citiesOnly([obj]) + ", " + upperCasingStates([obj["state"]])
        );
    });
}