import {createQuery, createWorld, World} from "@javelin/ecs";
import {netSystem, stopData, useNet} from "./net";
import {addElementsSystem, renderSystem, updateUI, bomberAnimationSystem, debugSystem} from "./systems";
import {GameElement} from "./schemas";
import {ROOT} from "./globals";
import assert from "assert";
import { lobby_interval, timerInterval } from "./scenes";


export let world = createWorld<number>({
	systems: [
		netSystem,
		addElementsSystem,
		renderSystem,
		updateUI,
		bomberAnimationSystem,
		debugSystem,
		resetWorldSystem,
	]
})

let doReset = false
let resetFn = () => {}
const toRemove = createQuery(GameElement).bind(world)
function resetWorldSystem(world: World) {
	if (!doReset) return
	doReset = false

	console.log("resetting world")

	ROOT.dataset.scene = ""

	toRemove.bind(world)((ent, [elem]) => {
		(elem as HTMLElement).remove()
	})

	const toDestroy = Array<number>()
	world.storage.archetypes.slice(1).forEach((arch) => {
		arch.entities.forEach((eid) => {
			toDestroy.push(eid)
		})
	})

	toDestroy.forEach(eid => {
		world.destroy(eid)
	})

	// world.storage.clear()

	resetFn()
}

export function resetWorld(callback: ()=>void) {
	stopData()
	doReset = true
	clearInterval(timerInterval)
	clearInterval(lobby_interval)
	resetFn = callback
}
