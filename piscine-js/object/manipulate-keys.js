const filterKeys = (obj, func) => {
  const res = {};
  Object.keys(obj).forEach((key) => {
    if (func(key)) res[key] = obj[key];
  });
  return res;
};
const mapKeys = (obj, func) => {
  const res = {};
  Object.keys(obj).forEach((key) => {
    res[func(key)] = obj[key];
  });
  return res;
};
const reduceKeys = (obj, func, acc) => {
  const res = {
    ...obj,
  };
  let ac = Object.keys(res)[0];
  if (acc || acc == 0) {
    return Object.keys(res).reduce(func, acc);
  } else {
    acc = "";
    return acc + Object.keys(res).reduce(func);
  }
};
