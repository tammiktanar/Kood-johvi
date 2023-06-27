let fold = (arr, func, acc) => {
    arr.forEach((val, index, arr) => (acc = func(acc, val)));
    return acc;
  };
  let foldRight = (arr, func, acc) => {
    let newarr = arr.slice();
    newarr.reverse();
    newarr.forEach((val, index, arr) => (acc = func(acc, val)));
    return acc;
  };
  let reduce = (arr, func, acc) => {
    typeof acc !== "undefined"
      ? arr.forEach((val, index, arr) => (acc = func(acc, val)))
      : (acc = reduce(arr.slice(1), func, arr[0]));
    return acc;
  };
  let reduceRight = (arr, func, acc) => {
    typeof acc !== "undefined"
      ? arr.forEach((val, index, arr) => (acc = func(acc, val)))
      : (acc = reduceRight(arr.slice(0, -1).reverse(), func, arr.slice(-1)[0]));
    return acc;
  };
  