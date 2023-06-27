let i = -1
function queryServers(serverName, q) {    
    console.log(i)
    let mainURL = "/"+serverName+"?q="+q
    let backURL = "/"+serverName+"_backup?q="+q
    i++
    if (i == 0) {
        return mainURL
    }
    if (i == 1) {
        return backURL
    }
}

function gougleSearch(q) {
    i++
    if (i == 2) {
        return {
            web: "/web?q="+q,
            image: "/image_backup?q="+q,
            video: "/video?q="+q
          }
    }
    if (i == 3) {
        return {
            web: "/web_backup?q="+q,
            image: "/image?q="+q,
            video: "/video_backup?q="+q
          }
    }
    if (i == 4) {
        return new Promise(function(resolve, reject) {
            setTimeout(function() {
                reject(Error('timeout'))
              }, 80)
        })
    }

}
