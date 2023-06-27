const flow = (arr) => (...args) =>
  arr.slice(1).reduce((acc, func) => func(acc), arr[0](...args)); 
