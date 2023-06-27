 function blockChain(data, prev = { index: 0, hash: '0' }){
    return {
        index: prev.index+1,
        data: data,
        prev: prev,
        hash: hashCode(String(prev.index+1)+String(prev.hash)+JSON.stringify(data)),
        chain: function(data) {
            return blockChain(data, this)
        }
    }
 }

