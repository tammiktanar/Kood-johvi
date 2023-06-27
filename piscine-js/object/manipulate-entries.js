const filterEntries = (obj, func) => {
  let newObj = {};
  Object.entries(obj)
    .filter(func)
    .map((each) => {
      newObj[each[0]] = each[1];
    });
  return newObj;
};

const mapEntries = (obj, func) => {
  let newObj = {};
  Object.entries(obj)
    .map(func)
    .map((each) => {
      newObj[each[0]] = each[1];
    });
  return newObj;
};

const reduceEntries = (obj, func, acc) => {
  const newObj = {
    ...obj,
  };
  if (acc || acc == 0) {
    return Object.entries(newObj).reduce(func, acc);
  } else {
    acc = "";
    return acc + Object.entries(newObj).reduce(func);
  }
};

const totalCalories = (obj) => {
  return reduceEntries(
    mapEntries(obj, ([k, v]) => [
      `${k}`,
      (Math.round(nutritionDB[k]["calories"] * v) / 1000) * 10,
    ]),
    (acc, [k, v]) => acc + v,
    0
  );
};

const lowCarbs = (obj) => {
  return filterEntries(
    obj,
    ([k, v]) => (v / 100) * nutritionDB[k]["carbs"] < 50
  );
};
const cartTotal = (obj) => {
  return mapEntries(obj, ([k, v]) => {
    let newObj = {};
    for (let [key, val] of Object.entries(nutritionDB[k]))
      newObj[key] = parseFloat(((val * v) / 100).toFixed(3));
    return [k, newObj];
  });
};

// Finish your groceries!!!

// Create 3 functions that works like the .filter, .map and .reduce array method but for the entries of the grocery cart.

// filterEntries filters using both key and value.
// mapEntries changes either the key or the value or both.
// reduceEntries reduce over entries.
// Create 3 other functions that use your previously create functions:

// totalCalories that will return the total calories of a cart
// lowCarbs that will leave only items that total carbs are lower than 50grams
// cartTotal that will give you the right amount of calories, proteins, ..., of all items in your grocery cart.
