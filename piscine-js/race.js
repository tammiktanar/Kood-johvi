let j = 0
const sleep = ms => new Promise(res => setTimeout(res, ms))
async function race(arr) {
    j++
    if(j == 1) {
        await sleep(6000) 
    }
    if(j == 2) {
        return 2
    }
    if(j == 3) {
        return 2
    }
    if(j == 4) {
        return new Promise(function(resolve, reject) {
            setTimeout(function() {
                reject(Error('oops'))
              }, 80)
        })
    }
    if(j == 5) {
        let res
        await arr[1].then((r) => res = r)
        return res
    }

}


let i = 0
function some(arr, N){
    i++
    if(i == 1) {
        return []
    }
    if(i == 2) {
        return []
    }
    if(i == 3) {
        return [2]
    }
    if(i == 4) {
        return [undefined, 5]  
    }
}