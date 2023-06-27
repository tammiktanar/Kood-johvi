function throttle(callback, interval) {
    let enableCall = true;
  
    return (...args)=> {
      if (!enableCall) return;
  
      enableCall = false;
      callback.apply(this, args);
      setTimeout(() => enableCall = true, interval);
    }
}
function opThrottle(func, wait, option = {}) {
    let timer = null;
    let last = null;
   
    function setTimer() {
      timer = setTimeout(timerFunc, wait);
    }
  
    function timerFunc() {
      timer = null;
      if (last && option.trailing) {
        func.apply(last.context, last.args) 
        setTimer();
      } 
       last = null;
    }
  
    return function throttled(...args) {
      if (timer === null) {
        option.leading ? func.apply(this, args) : last = {args, context: this};
        setTimer();
      } else {
        last = {args, context: this} ;
      }
    }
}
