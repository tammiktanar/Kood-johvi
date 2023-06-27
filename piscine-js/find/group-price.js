
/*
Instructions
Create a groupPrice function, that can find a price in a given string. The function has to return an array of arrays with the full match of the price ($43.99) and the groups of that price, the first group(43) and the second group (99). If there is no match the function returns an empty array. Example: "The price is USD12.31" Expected output: [["USD12.31","12","31"]]
*/

function groupPrice(str) {
    var result = /(USD)\d*\.\d*|[$]\d*\.\d*/g;
  
    var regex1 = /\d*(?=\.)/;
    var regex2 = /\d*$/;
    var match = str.match(result);
    var arr = [];
    
    if (match !== null) {
      for (let item of match) {
          let array = []
          array.push(item);
          array.push(regex1.exec(item)[0]);
          array.push(regex2.exec(item)[0]);
        arr.push(array);
      }
    }
    return arr;
  }