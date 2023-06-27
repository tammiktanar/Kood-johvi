import {
	component,
	createQuery,
	createWorld,
	observe,
	toComponent,
	World
} from "@javelin/ecs";
import {
	physicsSystem,
	netcodeSystem,
	killSwitchSystem,
	killDisconnected, collisionSystem, inputSystem, timerSystem, cacheSystem,
	bombSystem, explosionSystem, getInitialPacket, gameFlowSystem, closingSystem
} from "./systems";
import {gameScene} from "./scenes";
import {TickData} from "./net";
import {Socket} from "socket.io";
import {Bomber, Connection, Input, Name, Team} from "./schemas";
import {removeNamedListener} from "./lib";
import {debugSystem} from "./systems";
import {encode, Message} from "@javelin/net";


export function newWorld(sockets: Socket[]): World<TickData> {
	const world = createWorld<TickData>({
		systems: [
			inputSystem,
			timerSystem,

			physicsSystem,
			collisionSystem,

			cacheSystem,

			explosionSystem,
			bombSystem,

			closingSystem,

			debugSystem,
		]
	})

	// Initialize game world
	gameScene(world)
	world.step({dt: 0, kill: false, now: 0, tick: 0})

	// Initialize connections
	addSockets(world, sockets)
	world.step({dt: 0, kill: false, now: 0, tick: 0})

	// Add systems that require initialized world
	world.addSystem(gameFlowSystem)
	world.addSystem(killSwitchSystem)
	world.addSystem(killDisconnected)
	world.addSystem(netcodeSystem)


	return world
}

const bombers = createQuery(Team, Bomber)
function addSockets(world: World, sockets: Socket[]) {
	bombers.bind(world)((e, [t]) => {
		if (t.num <= sockets.length) {
			const socket = sockets[t.num-1]

			t.color = socket.data["color"]
			world.attach(e, component(Name, {s: socket.data["username"]}))

			socket.data["eid"] = e
			world.attach(e, toComponent(socket, Connection))

			initSocket(world, e, socket)
		} else {
			world.destroy(e)
		}
	})
}

function initSocket(world: World, eid: number, socket: Socket) {
	const input = observe(world.get(eid, Input))

	socket.on("input-up", (active: boolean) => {
		input.up = active
	})

	socket.on("input-down", (active: boolean) => {
		input.down = active
	})

	socket.on("input-left", (active: boolean) => {
		input.left = active
	})

	socket.on("input-right", (active: boolean) => {
		input.right = active
	})

	socket.on("input-bomb", (active: boolean) => {
		input.bomb = active
	})

	socket.on("input-walk", (active: boolean) => {
		input.walk = active
	})

	socket.on("request-ecs-data-init", () => {
		socket.emit("ecs-data-init", encode(getInitialPacket(world) as Message))
	})

	const dcFn = (done: unknown) => {
		world.detach(eid, Connection)
		uninitSocket(world, socket, true)

		if (typeof done === "function") done()
	}

	socket.data["game-dc-fn"] = dcFn

	socket.on("leave-game", dcFn)

	socket.on("disconnect", dcFn)
}

export function uninitSocket(world: World, socket: Socket, joinGlobal: boolean) {
	if ( !socket.rooms.has("game-"+world.id) ) return

	removeNamedListener(socket, "input-up")
	removeNamedListener(socket, "input-down")
	removeNamedListener(socket, "input-left")
	removeNamedListener(socket, "input-right")
	removeNamedListener(socket, "input-bomb")
	removeNamedListener(socket, "input-walk")
	removeNamedListener(socket, "leave-game")
	removeNamedListener(socket, "request-ecs-data-init")

	if (socket.data["game-dc-fn"]) {
		socket.off("disconnect", socket.data["game-dc-fn"])
	}

	socket.leave("game-"+world.id)

	if (joinGlobal) {
		socket.join("global-chat")
		socket.data["chat-room"] = "global-chat"
		socket.emit("join-global-chat")
	}
}
