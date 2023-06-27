const defaultCurry = (first) => {
  return (second) => {
    let newObj = { ...first };
    Object.entries(second).forEach(([key, _]) => (newObj[key] = second[key]));
    return newObj;
  };
};
const mapCurry = (func) => {
  return (obj) => {
    let newObj = {};
    Object.entries(obj)
      .map(func)
      .map((each) => {
        newObj[each[0]] = each[1];
      });
    return newObj;
  };
};
const reduceCurry = (func) => {
  return (obj, acc) => {
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
};
const filterCurry = (func) => {
  return (obj) => {
    let newObj = {};
    Object.entries(obj)
      .filter(func)
      .map((each) => {
        newObj[each[0]] = each[1];
      });
    return newObj;
  };
};
const reduceScore = (obj, acc) => {
  return reduceCurry((acc, [_, val]) => {
    return acc + val.shootingScore + val.pilotingScore;
  })(filterCurry(([_, v]) => v["isForceUser"])(obj), acc);
};
const filterForce = (obj) => {
  return filterCurry(([_, v]) => v.shootingScore >= 80)(
    filterCurry(([_, v]) => v["isForceUser"])(obj)
  );
};
const mapAverage = (obj) => {
  return mapCurry(([k, v]) => {
    let newObj = { ...v };
    newObj["averageScore"] = (v.shootingScore + v.pilotingScore) / 2;
    return [k, newObj];
  })(obj);
};
