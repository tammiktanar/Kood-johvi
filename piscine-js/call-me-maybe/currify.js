/*
const mult2 = (el1,el2) => el1 * el2
console.log(mult2(2,2)) // result expected 4


const mult2Curried = currify(mult2)

console.log(mult2Curried(2)(2))
*/
function currify(fn, ...values) {
  return (...next) => {
    if (Number.isNaN(fn(...values, ...next))) {
      return currify(fn, ...values, ...next);
    } else {
      return fn(...values, ...next);
    }
  };
}
