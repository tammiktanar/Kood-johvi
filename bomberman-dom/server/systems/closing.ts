import {component, ComponentOf, createQuery, World} from "@javelin/ecs";
import {
	Closing,
	Collider,
	GrayWall,
	GridPosition, Hidden,
	Size,
	TimerFinished,
	Wall
} from "../schemas";
import {FieldNumber} from "@javelin/core";
import {addTimer} from "./timer";
import {TickData} from "../net";
import {
	BLOCK_SIZE,
	CLOSING_DELAY,
	CLOSING_LAYERS,
	MAP_HEIGHT,
	MAP_WIDTH
} from "../env";
import {cloneComponent} from "../lib";
import {Caches, CFlagBlock, useCaches} from "../caches";

const wallClosing = createQuery(GridPosition, Closing, Wall, TimerFinished)
const grayWallClosing = createQuery(GridPosition, Closing, GrayWall, TimerFinished)
const grayWalls = createQuery(GrayWall, Closing)
export function closingSystem(world: World<TickData>) {
	const caches = useCaches()

	wallClosing((eid, [pos, closing]) => {
		world.detach(eid, TimerFinished)

		const newPos = getNextPos(pos, closing)
		// createClosingWall(world, newPos, closing)
		createClosingWall(world, newPos, closing)
	})

	grayWallClosing((eid, [pos, closing]) => {
		world.detach(eid, TimerFinished)

		const newPos = getNextPos(pos, closing)
		createClosingGrayWall(world, newPos, closing, caches)
	})
}

enum ClosingDir {
	Right,
	Down,
	Left,
	Up,
}

const WH_DIFF = MAP_WIDTH - MAP_HEIGHT

type PosComponent = ComponentOf<{x: FieldNumber, y: FieldNumber}>
type ClosingComponent =  ComponentOf<{dir: FieldNumber}>

export function createClosingWall(world: World<TickData>, pos: PosComponent, closing: ClosingComponent) {
	// Check if creation of gray walls has started
	for (const [entities] of grayWalls) {
		if (entities.length > 0) {
			return
		}
	}

	const eid = world.create(
		component(Wall),
		pos,
		cloneComponent(Closing, closing),
		component(Collider),
		component(Size, {width: BLOCK_SIZE, height: BLOCK_SIZE})
	)

	// Stop condition
	if (!(pos.x === CLOSING_LAYERS && pos.y === CLOSING_LAYERS + 1))
		addTimer(world, eid, CLOSING_DELAY)

	return eid
}

export function createClosingGrayWall(world: World<TickData>, pos: PosComponent, closing: ClosingComponent, caches: Caches) {
	const eid = world.create()
	world.attachImmediate(eid, [
		component(GrayWall),
		pos,
		cloneComponent(Closing, closing),
		component(Size, {width: BLOCK_SIZE, height: BLOCK_SIZE})
	])

	// Stop condition
	if (!(pos.x === 8 && pos.y === 6)) {
		addTimer(world, eid, 0.025)
	} else {
		world.latestTickData.kill = true
	}

	if (caches.block.flags[pos.y][pos.x] === CFlagBlock.Wall) {
		world.attachImmediate(eid, [component(Hidden)])
	}

	return eid
}

function getNextPos(pos: PosComponent, closing: ClosingComponent): PosComponent {
	switch (closing.dir) {
		case ClosingDir.Right:
			if (pos.x === MAP_WIDTH - pos.y - 1)
				closing.dir = (closing.dir + 1) % 4
			break

		case ClosingDir.Down:
			if (pos.y === pos.x - WH_DIFF)
				closing.dir = (closing.dir + 1) % 4
			break

		case ClosingDir.Left:
			if (pos.x === MAP_HEIGHT - pos.y - 1)
				closing.dir = (closing.dir + 1) % 4
			break

		case ClosingDir.Up:
			if (pos.y === pos.x + 1)
				closing.dir = (closing.dir + 1) % 4
			break
	}
	// console.log(closing.dir, JSON.parse(JSON.stringify(pos)))

	const offset = {x: 0, y: 0}
	const dir = closing.dir
	switch (dir) {
		case ClosingDir.Right: offset.x =  1; break
		case ClosingDir.Down:  offset.y =  1; break
		case ClosingDir.Left:  offset.x = -1; break
		case ClosingDir.Up:    offset.y = -1; break
	}

	const newPos = cloneComponent(GridPosition, pos)
	newPos.x += offset.x
	newPos.y += offset.y
	return newPos
}
