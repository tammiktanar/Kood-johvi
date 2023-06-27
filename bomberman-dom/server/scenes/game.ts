import {component, World} from "@javelin/ecs";
import {setupMap} from "./map";
import {Bomber, Lives} from "../schemas";


export const gameScene = (world: World) => {
	// clearElements(world)
	// resetWorld(world)

	// world.root.dataset.scene = "game-scene"
	// root.focus()
	// initWorld(world)

	let map = {
		crates: 75,
		data: [
			"###############",
			"#1_x.......x_4#",
			"#_#.#.#.#.#.#_#",
			"#x...........x#",
			"#.#.#.#.#.#.#.#",
			"#.............#",
			"#.#.#.#.#.#.#.#",
			"#.............#",
			"#.#.#.#.#.#.#.#",
			"#x...........x#",
			"#_#.#.#.#.#.#_#",
			"#3_x.......x_2#",
			"###############",
		],
	}

	setupMap(world, map, bomberSetupHandler)
	// setupSideMenu(world)
}

function bomberSetupHandler(world: World, eid: number) {
	world.attach(eid,
		component(Bomber, {speed: 90, power: 2, bombCount: 1}),
		component(Lives, {current: 3, max: 3}),
	)
}

