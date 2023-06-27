const filterValues = (obj, func) => {
  const res = {};
  Object.keys(obj).forEach((key) => {
    if (func(obj[key])) res[key] = obj[key];
  });
  return res;
};
const mapValues = (obj, func) => {
  const res = {
    ...obj,
  };
  Object.keys(res).forEach((key) => {
    res[key] = func(res[key]);
  });
  return res;
};
const reduceValues = (obj, func, acc) => {
  acc ? acc : (acc = 0);
  const res = {
    ...obj,
  };
  return acc + Object.values(res).reduce(func);
};
