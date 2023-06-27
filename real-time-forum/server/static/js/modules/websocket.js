import chat from "../main.js"

const dev = false
let path = "ws://" + window.location.host + "/message"
if (location.protocol == "https:") path = "wss://" + window.location.host + "/message"

let ws = new WebSocket(path)
let currentUserID

export async function openWebsocket() {
    ws = new WebSocket(path)

    ws.onmessage = event => {
        let data = JSON.parse(event.data)
        let kind = data.kind;
        let content = data.content;
        switch (kind) {
            case 1:
                if(dev) console.log(kind, content)
                if (content.id == 0) {
                    closeWebSocket()
                    if(dev) console.log(kind, "Not logged in")
                } else {
                    currentUserID = content.id
                    if(dev) console.log(kind, "Logged in")
                    loadChatIntoBody()
                }
                break;

            case 101: // Send online users
                if(dev) console.log(kind, "Send online users", content)
                content.forEach(user => {
                    if(dev) console.log(kind, user)
                    if (user.user_id === currentUserID) return
                    chat.userOnline(user)
                });
                
                
                break;
                
            case 102: // Send known users
                if(dev) console.log(kind, "Send known users", content)
                content.forEach(content => {
                    if(dev) console.log(kind, content)
                    if (content.user.user_id === currentUserID) return
                    chat.userKnown(content.user, content.online)
                });

                requestOnlineUsers()
                break;

            case 103: // Send online state
                if(dev) console.log(kind, "Send online state", content)

                if (content.user.user_id === currentUserID) return
                chat.userKnown(content.user, content.online ? chat.userOnline(content.user) : chat.userOffline(content.user))

                break;
        
            case 104: // User went online
                if(dev) console.log(kind, "User went online", content)
                chat.userOnline(content)
                break;

            case 105: // User went offline
                if(dev) console.log(kind, "User went offline", content)
                chat.userOffline(content)
                break;

            case 106: // Receive message
                if(dev) console.log(kind, "Receive message", content)
                
                    if (dev) console.log(kind, content)
                    if (document.hidden || (chat.activeUser != content.receiver && chat.activeUser != content.sender)) {
                        chat.playSound()
                        return
                    }
                    let incoming  = (content.receiver == currentUserID)
                    let otherUserID = incoming ? content.sender :  content.receiver
                    if (!chat.bumpKnownUser(otherUserID)) requestOnlineState(otherUserID, true)
                    let date = new Date(content.date)
                    let dateString = (date.getHours() + ":" + date.getMinutes() + " " + date.getDate() + "/" + date.getMonth() + "/" + date.getFullYear())
                    chat.addNewMessage(incoming, content.text, dateString)

                break;

            case 107: // Send message history
                if(dev) console.log(kind, "Send message history", content)
                if (content.history == null)  {
                    chat.noMoreHistory()
                    return
                }
                if (chat.activeUser != content.user_id) return 

                content.history.forEach(message => {
                    if (dev) console.log(kind, message)
                    let incoming  = (message.receiver == currentUserID)
                    let date = new Date(message.date)
                    let dateString = (date.getHours() + ":" + date.getMinutes() + " " + date.getDate() + "/" + date.getMonth() + "/" + date.getFullYear())
                    chat.addOldMessage(incoming, message.text, dateString, message.message_id)
                });
                chat.resetHistoryCooldown()
                break;

            case 108: // Is typing
                if(dev) console.log(kind, "Is typing", content)
                if (content.sender != chat.activeUser) return
                
                chat.isTyping() 
                break;

            default:
                if(dev)console.log("no kind", event.data)

                break;
        }
    }


    ws.onopen = () => {
        requestKnownUsers()
    }

    chat.onSendMessage = (text, receiverID) => {
        sendMessage(receiverID, text)
    }

    chat.onLoadMessages = (userID, fromMessage) => {
        requestMessageHistory(userID, fromMessage)
    }

    chat.onSearchUser = (username) => {
        requestOnlineState(username, false)
    }

    chat.onTyping = (userID) => {
        startedTyping(userID)
    }

}

function wsIsOpen(){
    if (ws.readyState === WebSocket.OPEN) return true
    return false
}

export function requestOnlineUsers(){ // Request online users
    if (!wsIsOpen) return
    let payload = {
        kind: 11,
    }

    if (dev) console.log(payload.kind, "Request online users")
    ws.send(JSON.stringify(payload))
}

export function requestKnownUsers(){ // Request known users
    if (!wsIsOpen) return
    let payload = {
        kind: 12,
    }
    
    if (dev) console.log(payload.kind, "Request known users")
    ws.send(JSON.stringify(payload))
}

export function requestOnlineState(userinfo, useID){ // Request user online state
    if (!wsIsOpen) return
    let payload = {
        kind: 13,
        content: {}
    }

    if (useID) {
        payload.content.id = userinfo
    } else {
        payload.content.name = userinfo
    }

    if (dev) console.log(payload.kind, "Request user online state", payload.content)
    ws.send(JSON.stringify(payload))
}

export function sendMessage(userID, message){ // Send message 
    if (!wsIsOpen) return
    if (message == "") return
    let payload = {
        kind: 16,
        content: {
            receiver: userID,
            text: message
        }
    }

    if (dev) console.log(payload.kind, "Send message", payload.content)
    ws.send(JSON.stringify(payload))
}

export function requestMessageHistory(userID, messageID){ // Request message history
    if (!wsIsOpen) return
    let payload = {
        kind: 17,
        content: {
            receiver: userID,
            from_message: messageID
        }
    }

    if (dev) console.log(payload.kind, "Request message history", payload.content)
    ws.send(JSON.stringify(payload))
}

export function startedTyping(userID){ // Started typing
    if (!wsIsOpen) return
    let payload = {
        kind: 18,
        content: {
            receiver: userID,
        }
    }


    if (dev) console.log(payload.kind, "Started typing", payload.content)
    ws.send(JSON.stringify(payload))
}

export function loadChatIntoBody(){
    if (wsIsOpen()) document.body.appendChild(chat.element)
}

export function closeWebSocket(){
    if (dev) console.log("Websocket closed")
    ws.close()
    currentUserID = 0
}

openWebsocket()
