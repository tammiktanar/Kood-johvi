import {component, observe, World} from "@javelin/ecs";
import {
	Bomb,
	Bomber,
	GridPosition,
	Owner,
	Position,
	Size,
	Team
} from "../schemas";
import {CFlagBomb, Cache, Caches, CFlagBlock} from "../caches";
import {BLOCK_SIZE} from "../env";
import {addTimer} from "../systems";
import {TickData} from "../net";
import {cloneComponent} from "../lib";

export const spawnBomb = (world: World<TickData>, caches: Caches, owner: number) => {
	const pos = world.get(owner, Position)
	const size = world.get(owner, Size)

	const middle = {x: pos.x + size.width/2, y: pos.y + size.height/2}
	const gridPos = {x: pxToGrid(middle.x), y: pxToGrid(middle.y)}

	if (caches.bomb.flags[gridPos.y][gridPos.x] !== CFlagBomb.Empty) {
		// Already exists a bomb here
		return
	}

	if (caches.block.flags[gridPos.y][gridPos.x] !== CFlagBlock.Empty) {
		// Can't place a bomb inside a wall
		return
	}

	const bomber = world.get(owner, Bomber)
	const power = bomber.power

	const team = world.get(owner, Team)

	observe(bomber).bombCount -= 1
	makeBombEntity(world, gridPos, power, team, owner)

	return world
}

function pxToGrid(px: number): number {
	return Math.floor(px / BLOCK_SIZE)
}

function makeBombEntity(world: World<TickData>, gridPos: {x: number, y: number}, power: number, team: {num: number, color: string}, owner: number) {
	const bomb = world.create(
		component(Bomb, {power}),
		cloneComponent(Team, team),
		component(Owner, {eid: owner}),
		cloneComponent(GridPosition, gridPos),
		component(Size, {width: BLOCK_SIZE, height: BLOCK_SIZE}),
	)

	addTimer(world, bomb, 2.5)
}
