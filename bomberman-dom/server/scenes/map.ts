import {component, World} from "@javelin/ecs";
import {Collider, Crate, GridPosition, Size, Wall} from "../schemas";
import {BLOCK_SIZE} from "../env";
import {addBomber} from "./bomber";

interface Map {
	crates: number,
	data: string[]
}

interface BomberSetupHandler {
	(world: World, eid: number): void
}

export const setupMap = (world: World, map: Map, bomberSetup: BomberSetupHandler) => {

	const possibleCrates = [];

	for (let y = 0; y < map.data.length; y++) {
		for (let x = 0; x < map.data[y].length; x++) {
			switch (map.data[y][x]) {
				case '#': // Wall
					addWall(world, x, y)
					break

				case '.': // Potential crate
					possibleCrates.push([x, y])
					break

				case 'x': // Guaranteed crate
					addCrate(world, x, y)
					break

				// Player spawns
				case '1':
				case '2':
				case '3':
				case '4':
					const eid = addBomber(world, parseInt(map.data[y][x]), x, y)
					bomberSetup(world, eid)
					break

				default: // Always empty
					break
			}
		}
	}

	// Place X amount of crates in empty slots
	const CRATE_COUNT = map.crates
	const count = Math.min(possibleCrates.length, CRATE_COUNT)
	for (let i = 0; i < count; i++) {
		const [[x, y]] = possibleCrates.splice(Math.floor(Math.random() * possibleCrates.length), 1)
		addCrate(world, x, y)
	}
}

const addWall = (world: World, x: number, y: number) => {
	world.create(
		component(Wall),
		component(GridPosition, {x: x, y: y}),
		component(Size, {width: BLOCK_SIZE, height: BLOCK_SIZE}),
		component(Collider)
	)
}


const addCrate = (world: World, x: number, y: number) => {
	world.create(
		component(Crate),
		component(GridPosition, {x: x, y: y}),
		component(Size, {width: BLOCK_SIZE, height: BLOCK_SIZE}),
		component(Collider)
	)
}
