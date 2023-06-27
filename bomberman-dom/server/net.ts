import express from 'express';
import {Server, Socket} from "socket.io";
import { createServer } from "http";
import {randomIntFromInterval, socketName} from "./lib";
import {newWorld, uninitSocket} from "./world";
import {Clock, createHrtimeLoop} from "@javelin/hrtime-loop";
import {CORS_ORIGIN, GAME_END_DELAY, PRODUCTION, TICK_RATE} from "./env";
import {LobbyManager} from "./lobby";
import path from "path";

const app = express();

if (PRODUCTION) {
	app.use("/", express.static(path.join(__dirname, "../client/dist")));
}

export const httpServer = createServer(app);

const cors = CORS_ORIGIN ? CORS_ORIGIN.split("|") : true
console.log("cors origins:", cors)

export const io = new Server(httpServer, {
    cors: {
        origin: cors,
        methods: ["GET", "POST"]
    }
});


///////////////////
// Lobby section //
///////////////////

const lobbies: Map<number, LobbyManager> = new Map()
const createLobby = (): LobbyManager => {
	let id = 0
	while (true) {
		id = randomIntFromInterval(10000000, 99999999)
		if (!lobbies.has(id)) break
	}

	const lobby = new LobbyManager(id,
		(members) => {
			lobbies.delete(id)
			startGame(members)
		},
		() => {
			console.log("closing lobby", id)
			lobbies.delete(id)
		}
	)

	lobbies.set(id, lobby)

	return lobby
}

io.on('connect', (socket) => {
	console.log(socketName(socket), "connected")

	socket.data["username"] = ""
	socket.data["color"] = "#ff00ff"

	socket.on("set-username", (name: string) => {
		console.log(socket.id, "is renamed to", name)
		socket.data["username"] = name
	})

	socket.on("set-color", (color: string) => {
		// console.log(socketName(socket), "has a color of", color)
		socket.data["color"] = color
	})

	socket.on("create-lobby", (success: ()=>void) => {
		const lobby = createLobby()
		lobby.join(socket)
		success()
		console.log(socketName(socket), "created lobby", lobby.id)
	})

	socket.on("join-lobby", (id: number, result: (success: boolean)=>void) => {
		if (!lobbies.has(id)) {
			result(false)
			return
		}

		const lobby = lobbies.get(id)
		if (!lobby?.tryJoin(socket)) {
			result(false)
			return
		}

		result(true)
		console.log(socketName(socket), "joined lobby", lobby.id)
	})

	socket.on("quick-play", (success: ()=>void) => {
		for (const lobby of lobbies.values()) {
			if (lobby.tryJoin(socket)) {
				success()
				console.log(socketName(socket), "quick play joined lobby", lobby.id)
				return
			}
		}

		const lobby = createLobby()
		lobby.join(socket)
		success()
		console.log(socketName(socket), "quick play created lobby", lobby.id)
	})

	socket.on('disconnect', () => {
		console.log(socketName(socket), "disconnected")
	})
})


//////////////////
// Game section //
//////////////////
function startGame(members: Socket[]) {
	const world = newWorld(members)
	console.log("starting game", world.id, "with", members.map(mem => mem.data["username"]))

	members.forEach(socket => {
		socket.join("game-"+world.id)

		socket.leave("global-chat")
		socket.data["chat-room"] = "game-"+world.id
		socket.emit("join-game-chat")
	})
	io.in("game-"+world.id).emit("game-starting")

	let gameEndStarted = false

	// Game loop
	const start = process.hrtime.bigint()
	const NS_IN_MS = 1_000_000
	const loop = createHrtimeLoop((clock) => {
		const tickData: TickData = {...clock, kill: false, now: Number(clock.now.valueOf() - start)/NS_IN_MS}
		world.step(tickData)

		if (tickData.kill && !gameEndStarted) {
			gameEndStarted = true

			setTimeout(() => {
				console.log("ending game", world.id)

				// Get sockets that are still connected
				const clients = io.sockets.adapter.rooms.get("game-"+world.id)
				if (clients && clients.size > 0) {
					const lobby = createLobby()
					clients.forEach(clientId => {
						const clientSocket = io.sockets.sockets.get(clientId)
						if (!clientSocket) return

						uninitSocket(world, clientSocket, false)
						lobby.join(clientSocket)

						clientSocket.emit("game-ending", lobby.id)
					})
				}

				loop.stop()
			}, GAME_END_DELAY*1000)
		}

	}, (1 / TICK_RATE) * 1000)

	loop.start()
}

export type TickData = Omit<Clock, 'now'> & {
	kill: boolean
	now: number
}

///////////////////
// Chat section //
///////////////////

io.on("connect", (socket) => {
	socket.join("global-chat")
	socket.data["chat-room"] = "global-chat"
	socket.emit("join-global-chat")

	socket.on("send-message", (message) => {
		if (socket.data["username"] === "") return

		const data = {
			message,
			username: socket.data["username"],
			color: socket.data["color"],
		}

		io.in(socket.data["chat-room"]).emit("receive-message", data)

		console.log(data.username, "sent a message in", socket.data["chat-room"], "with content", data.message)
	})
})
