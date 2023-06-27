let every = (arr, func) => {
  let res = true;
  arr.forEach((val, ind, arr) => (res = res && func(val)));
  return res;
};
let some = (arr, func) => {
  let res = false;
  arr.forEach((val, ind, arr) => (res = res || func(val)));
  return res;
};
let none = (arr, func) => {
  let res = true;
  arr.forEach((val, ind, arr) => (res = res && !func(val)));
  return res;
};
