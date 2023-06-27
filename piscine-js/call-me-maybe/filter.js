let filter = (arr, func) => {
    const res = [];
    arr.forEach((val, index, arr) => (func(val, index, arr) ? res.push(val) : 0));
    return res;
  };

  let reject = (arr, func) => {
    const res = [];
    arr.forEach((val, index, arr) => (func(val, index, arr) ? 0 : res.push(val)));
    return res;
  };
  
  let partition = (arr, func) => {
    const res1 = [];
    const res2 = [];
    arr.forEach((val, index, arr) =>
      func(val, index, arr) ? res1.push(val) : res2.push(val)
    );
    return [res1, res2];
  };
  