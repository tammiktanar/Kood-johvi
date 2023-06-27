import {component, World} from "@javelin/ecs";
import {
	Caches,
	CFlagBlock,
	CFlagBomb,
	CFlagExplosion,
} from "../caches";
import {Bomb, Explosion, GridPosition, Size, Team} from "../schemas";
import {addTimer} from "../systems";
import {TickData} from "../net";
import {BLOCK_SIZE} from "../env";
import {cloneComponent} from "../lib";

export const spawnExplosion = (world: World<TickData>, caches: Caches, x: number, y: number, power: number, team: {num: number, color: string}) => {
	if (caches.block.flags[y][x] !== CFlagBlock.Empty) {
		// Can't explode inside a wall
		return
	}

	caches.bomb.flags[y][x] = CFlagBomb.Empty

	makeExplosionEntity(world, caches, x, y, 0, 0, team)

	const b1 = makeExplosionBranch(world, caches, x, y, power, 0, team)
	const b2 = makeExplosionBranch(world, caches, x, y, power, 1, team)
	const b3 = makeExplosionBranch(world, caches, x, y, power, 2, team)
	const b4 = makeExplosionBranch(world, caches, x, y, power, 3, team)

	// Ignite other bombs that got caught in the explosion
	const otherBombs = [b1, b2, b3, b4]
		.filter((v): v is number => v != null)
		.map(eid => {return {
			eid: eid,
			pos: world.get(eid, GridPosition),
			bomb: world.get(eid, Bomb),
			team: world.get(eid, Team),
		}})

	// Remove them from the cache to prevent another bomb from igniting them
	for (const {pos} of otherBombs) {
		caches.bomb.flags[pos.y][pos.x] = 0
	}

	// Do the ignition
	for (const other of otherBombs) {
		spawnExplosion(world, caches, other.pos.x, other.pos.y, other.bomb.power, other.team)

		world.destroy(other.eid)
	}

	return world
}

const explosionTime = 1

function makeExplosionBranch(world: World<TickData>, caches: Caches, X: number, Y: number, power: number, direction: number, team: {num: number, color: string}): void | number {
	let [xStep, yStep] = [0, 0]
	switch (direction) {
		case 0:
			yStep = -1;
			break // Up
		case 1:
			xStep = 1;
			break // Right
		case 2:
			yStep = 1;
			break // Down
		case 3:
			xStep = -1;
			break // Left
	}

	const x = X + xStep
	const y = Y + yStep
	if (caches.block.flags[y][x] === 1) {
		return
	}

	if (doesExplosionCollide(world, caches, direction, x, y)) {
		return
	}

	for (let i = 1; i <= power; i++) {
		const x = X + xStep * i
		const y = Y + yStep * i

		const xNext = x + xStep
		const yNext = y + yStep

		let type = 1;

		// If hitting a bomb
		if (caches.bomb.flags[y][x] !== 0) {
			return caches.bomb.ents[y][x]
		}

		// If the next step would collide with another explosion
		if (doesExplosionCollide(world, caches, direction, xNext, yNext)) {
			type = 2
		}

		// If hitting a crate
		if (caches.block.flags[y][x] === 2) {
			type = 2

			const crate = caches.block.ents[y][x]
			addTimer(world, crate, explosionTime)
		}

		// If next is a wall
		if (caches.block.flags[yNext][xNext] === 1) {
			type = 2
		}

		// If end of explosion reach
		if (i === power) {
			type = 2
		}

		makeExplosionEntity(world, caches, x, y, type, direction, team)

		if (type === 2) break
	}
}

function isPerpendicular(dirA: number, dirB: number) {
	return dirA % 2 !== dirB % 2
}

// Checks if an explosion should collide with another one if it were to be spawned on the given coordinates.
function doesExplosionCollide(world: World, caches: Caches, myDirection: number, x: number, y: number): boolean {
	// console.log(caches)
	// If there is another explosion already
	if (y >= caches.explosion.flags.length || x >= caches.explosion.flags[0].length) {
		debugger
	}
	let flag = caches.explosion.flags[y][x]
	if (flag !== CFlagExplosion.Empty) {
		// If they were to collide inside a wall
		if (caches.block.flags[y][x] !== CFlagBlock.Empty) {
			return true
		}

		// If the explosions were to be parallel
		if (flag === CFlagExplosion.Cross ||
			!isPerpendicular(myDirection, world.get(caches.explosion.ents[y][x], Explosion).direction))
		{
			return true
		}
	}
	return false
}

function makeExplosionEntity(world: World<TickData>, caches: Caches, x: number, y: number, type: number, direction: number, team: {num: number, color: string}) {
	const eid = world.create()
	world.attachImmediate(eid, [
		component(Explosion, {type: type, direction: direction}),
		cloneComponent(Team, team),
		component(GridPosition, {x, y}),
		component(Size, {width: BLOCK_SIZE-8, height: BLOCK_SIZE-12}),
	])

	caches.explosion.flags[y][x] = type + 1
	caches.explosion.ents[y][x] = eid

	addTimer(world, eid, explosionTime)
}
