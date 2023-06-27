const isObject = (a) =>
  typeof a !== "object" ||
  a instanceof Set ||
  a instanceof RegExp ||
  a instanceof Array ||
  a instanceof Map
    ? false
    : true;

const replica = (target, ...other) => {
  other.forEach((ob) => {
    Object.entries(ob).forEach(([k, v]) => {
      if (isObject(v) && isObject(target[k])) {
        target[k] = replica(target[k], v);
      } else {
        target[k] = v;
      }
    });
  });

  return target;
};
