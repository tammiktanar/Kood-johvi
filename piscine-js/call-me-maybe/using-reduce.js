let adder = (arr, ext) =>
  ext !== undefined
    ? arr.reduce((acc, each) => acc + each, ext)
    : arr.reduce((acc, each) => acc + each, 0);

let sumOrMul = (arr, ext) =>
  ext !== undefined
    ? arr.reduce((acc, each) => (each % 2 === 1 ? acc + each : acc * each), ext)
    : arr.reduce((acc, each) => (each % 2 === 1 ? acc + each : acc * each), 0);
    
let funcExec = (func, x) => func.reduce((sum, current) => current(sum), x);