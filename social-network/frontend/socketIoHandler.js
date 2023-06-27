// socketIoHandler.js
import { Server } from 'socket.io';

import { io as client } from "socket.io-client";

const BACKEND_ADDRESS = process.env.VITE_BACKEND_ADDRESS ?? "localhost"
const BACKEND_PORT = "8888"
const BACKEND_URL = `http://${BACKEND_ADDRESS}:${BACKEND_PORT}`


export default function injectSocketIO(server) {
    const io = new Server(server);

    server.listeners('request').forEach(function(listener){
        server.removeListener('request', listener);
        server.on('request', (req, res) => {
            if (!req.url.startsWith("/notify")) {
                listener(req,res)
                return
            }

            let body = "";
            req.on('data', function(data) {
                body += data;
            });
            req.on('end', function() {
                try {
                    let {targets, message} = JSON.parse(body)
                    for (let t of targets) {
                        message.receiver = t;
                        io.in("user-"+t).emit("receiveMessage", message);
                    }
                } catch(e) {
                    console.error(e)
                }
                res.end()
            });
        });
    });

    io.on('connection', async (socket) => {
        try {
            socket.apiFetch = makeApiFetch(BACKEND_URL, socket.handshake.headers)
            let res = await socket.apiFetch("/api/user")
            if (!res.ok) return

            let user = await res.json()
            socket.userID = user.userID
            socket.join("user-"+socket.userID)

            socket.broadcast.emit("user-online", socket.userID)

            socket.on("sendMessage", async ({receiver, content, isGroup}) => {
                let message = {
                    sender: socket.userID,
                    receiver: receiver,
                    content: content,
                    isGroup: isGroup
                }

                const res = await socket.apiFetch("/api/message/send", {
                    method: "POST",
                    body: JSON.stringify(message)
                })

                if (!res.ok) {
                    console.error(`failed to fetch API: "${await res.text()}"`)
                    return
                }

                message = await res.json()

                if (message.isGroup) {
                    io.in("group-"+message.receiver).emit("receiveMessage", message)
                } else {
                    io.in("user-"+message.sender).in("user-"+message.receiver).emit("receiveMessage", message)
                }
            })

            socket.on("getConvos", async (callback) => {
                try {
                    const pGroup = socket.apiFetch("/api/group/my")
                    const pUser = socket.apiFetch("/api/user/known")
                    const [rGroup, rUser] = await Promise.all([pGroup, pUser])
                    const groups = await rGroup.json()
                    const users = await rUser.json()

                    groups.forEach(group => socket.join("group-"+group.groupID))
                    callback({
                        groups: groups,
                        users: users
                    })
                } catch (e) {
                    console.error(e)
                    callback({groups: [], users: []})
                }
            })
        } catch (e) {
            console.error("Websocket error:", e)
        }
    });

    console.log('SocketIO injected');
}


function makeApiFetch(baseURL, headers) {
    return (path, options = {}) => {
        console.log(`Websocket fetching "${path}" from backend`);
        (options.headers ??= {}).cookie = headers.cookie;
        return fetch(baseURL + path, options);
    }
}
