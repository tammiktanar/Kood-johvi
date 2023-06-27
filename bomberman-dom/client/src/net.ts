import {createEffect, World} from "@javelin/ecs";
import {io} from "socket.io-client";
import {createMessageHandler} from "@javelin/net";
import {resetWorld, world} from "./world";
import {
	createLobbyScreen,
	loadGameScene,
	createStartScreen
} from "./scenes";
import {production, ROOT, websocket_port} from "./globals";
import {playFX} from "./audio";
import {displayWinner} from "./scenes/winner";

export const socket = io(window.location.hostname + (production ? "" : `:${websocket_port}`))

let acceptData = false
export const useNet = createEffect(
	world => {
		const handler = createMessageHandler(world)
		socket.on("ecs-data-init", (data: ArrayBuffer) => {
			console.log("INITIALIZING WORLD")
			handler.push(data)
			acceptData = true
		})

		socket.on('ecs-data', (data: ArrayBuffer) => {
			// console.log("receiving data")
			if (acceptData) {
				handler.push(data)
			}
		})

		return () => {
			handler.system()
		}
	},
	{ shared: true },
)

export function stopData() {
	acceptData = false
}

export function netSystem(_world: World) {
	useNet()
}

socket.on('game-starting', () => {
	resetWorld(() => {
		socket.emit("request-ecs-data-init")
		loadGameScene(world)
	})
})

let winner: undefined | {team: number, color: string, name: string}

socket.on("set-winner", (newWinner: {team: number, color: string, name: string}) => {
	winner = newWinner
})

socket.on("game-ending", (_lobbyID) => {
	displayWinner(winner as {team: number, color: string, name: string})
	setTimeout(() => {
		resetWorld(() => {
			createLobbyScreen(world)
		})
	}, 3000)
})

socket.on("play-ding", () => {
	playFX("ding")
})

socket.on("disconnect", () => {
	resetWorld(() => {
		createStartScreen(world)
		alert("Connection error")
	})
})

////////////////////
// Input handling //
////////////////////
ROOT.addEventListener("keydown", e => handleKey(e, true))
ROOT.addEventListener("keyup", e => handleKey(e, false))

function handleKey(e: KeyboardEvent, pressed: boolean) {
	if (e.repeat) return
	if (e.target instanceof Element && e.target.tagName === "INPUT") return

	switch (e.code) {
		case "KeyW":
		case "ArrowUp":
			socket.emit("input-up", pressed)
			break

		case "KeyS":
		case "ArrowDown":
			socket.emit("input-down", pressed)
			break

		case "KeyA":
		case "ArrowLeft":
			socket.emit("input-left", pressed)
			break

		case "KeyD":
		case "ArrowRight":
			socket.emit("input-right", pressed)
			break

		case "Space":
			socket.emit("input-bomb", pressed)
			break

		case "ShiftLeft":
		case "ShiftRight":
			socket.emit("input-walk", pressed)
			break
	}
}

export function stopInputs() {
	const inputs = [
		"KeyW",
		"KeyA",
		"KeyS",
		"KeyD",
		"ArrowUp",
		"ArrowDown",
		"ArrowLeft",
		"ArrowRight",
		"Space",
		"ShiftLeft",
		"ShiftRight",
	]

	inputs.map(inp => new KeyboardEvent("keyup", {code: inp}))
		.forEach(ev => handleKey(ev, false))
}
