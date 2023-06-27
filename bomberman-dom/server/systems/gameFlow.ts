import {
	component,
	createEffect, createImmutableRef,
	createQuery, createRef,
	useRef,
	World
} from "@javelin/ecs";
import {io, TickData} from "../net";
import {
	Bomber, Closing, Collider,
	Connection, GridPosition,
	Lives, Name, Position, Size, Team, Timer,
	TimerFinished, Wall
} from "../schemas";
import {BLOCK_SIZE, CLOSING_DELAY, CLOSING_START, GAME_DURATION} from "../env";
import {addTimer} from "./timer";
import {createClosingGrayWall, createClosingWall} from "./closing";
import {Caches, useCaches} from "../caches";
import {Socket} from "socket.io";

export const useSetWinner = createEffect((world: World) => {
	let sent = false

	return () => (set: {team: number, color: string, name: string}) => {
		if (sent) return false

		io.in("game-"+world.id).emit("set-winner", set)
		return true
	}
})

const aliveBombers = createQuery(Team, Name, Bomber, Lives)

export function gameFlowSystem(world: World<TickData>) {
	const setWinner = useSetWinner()
	const caches = useCaches()
	const endStarted = useRef(false)

	// Check if someone has won
	if (!endStarted.value) {
		let count = 0
		let winner = {team: 0, color: "#ffffff", name: ""}

		aliveBombers((_, [t, n]) => {
			count++;
			winner = {team: t.num, color: t.color, name: n.s};
		})

		if (count <= 1) {
			endStarted.value = true
			setWinner(winner)
			startClosingGray(world, caches)
		}
	}

	// Game ended
	if (!endStarted.value && world.latestTickData.now >= GAME_DURATION*1000) {
		endStarted.value = true
		setWinner({team: 0, color: "#ffffff", name: ""})

		startClosingGray(world, caches)
	}

	// Walls closing
	const closing = useRef(false)
	if (!closing.value && world.latestTickData.now >= CLOSING_START*1000) {
		console.log("Starting closing in game", world.id)
		closing.value = true

		createClosingWall(world, component(GridPosition, {x: 1, y: 1}), component(Closing))
	}
}

function startClosingGray(world: World<TickData>, caches: Caches) {
	createClosingGrayWall(world, component(GridPosition, {x: 0, y: 0}), component(Closing), caches)
}


const query = createQuery(Connection)
export function killSwitchSystem(world: World<TickData>) {
	for (const [entities] of query) {
		if (entities.length > 0) {
			return
		}
	}

	world.latestTickData.kill = true
}

const disconnected = createQuery(Bomber, Lives).not(Connection)
export function killDisconnected(world: World) {
	disconnected((e) => {
		world.detach(e, Lives)
	})
}
