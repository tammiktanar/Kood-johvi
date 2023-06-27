let map = (arr, func) => {
    const res = [];
    arr.forEach((val, index, arr) => res.push(func(val, index, arr)));
    return res;
};

let flatMap = (arr, func) => {
    const res = map(arr, func);
    if (res[0] instanceof Array) {
      const nw = [];
      map(res, (each) => map(each, (x) => nw.push(x)));
      return nw;
    }
    return res;
};
  