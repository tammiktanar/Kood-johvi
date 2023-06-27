import {
	component,
	ComponentOf,
	createQuery,
	observe,
	Query,
	World
} from "@javelin/ecs";
import {
	Bomb,
	Bomber,
	Collider, Connection, Explosion, Ghost,
	GridPosition, IFrame, Lives,
	Position,
	PowerUp,
	Size, Velocity
} from "../schemas";
import {FieldNumber} from "@javelin/core";
import {BLOCK_SIZE} from "../env";
import {TickData} from "../net";
import {addTimer} from "./timer";
import {cloneComponent} from "../lib";
import {Socket} from "socket.io";


const bomberQuery = createQuery(Size, Position, Bomber, Lives)
const colliderQuery = createQuery(Size, GridPosition, Collider)
const bombQuery = createQuery(Size, GridPosition, Bomb)
const powerUpQuery = createQuery(Size, GridPosition, PowerUp)

const damageBomberQuery = createQuery(Size, Position, Bomber, Lives).not(IFrame)
const explosionQuery = createQuery(Size, GridPosition, Explosion)

type BomberQueryResult = [
	ComponentOf<{ width: FieldNumber, height: FieldNumber }>,
	ComponentOf<{ x: FieldNumber, y: FieldNumber }>,
	ComponentOf<{speed: FieldNumber, power: FieldNumber, bombCount: FieldNumber}>,
	...any[],
]
type ColliderQuery = Query<[
	{ width: FieldNumber, height: FieldNumber },
	{ x: FieldNumber, y: FieldNumber },
	...any[],
]>

export const collisionSystem = (world: World<TickData>) => {
	bomberQuery((e, components) => {
		doCollisions(world, e, components, colliderQuery, integratePositionsWithDamage)
		doCollisions(world, e, components, bombQuery, integratePositions)

		doCollisions(world, e, components, powerUpQuery, pickUpPowerUp)
	})

	damageBomberQuery((e, components) => {
		doCollisions(world, e, components, explosionQuery, explosionDamage)
	})
}

interface collisionResolver {
	// Return boolean determines if the bomber should continue to be tested after the first collision
	(world: World<TickData>, bomber: number, bComponents: BomberQueryResult, collider: number, a: Box, b: Box): boolean
}

function doCollisions(world: World<TickData>, bomber: number, bComponents: BomberQueryResult, colliders: ColliderQuery, resolver: collisionResolver) {
	for (const [entities, [sizes, gridPositions]] of colliders) {
		for (let i = 0; i < entities.length; i++) {
			const bomberPos = bComponents[1]
			const bomberSize = bComponents[0]
			const box1 = {
				minX: bomberPos.x,
				minY: bomberPos.y,
				maxX: bomberPos.x + bomberSize.width,
				maxY: bomberPos.y + bomberSize.height,
			}

			const colliderPos = gridPositions[i]
			const colliderSize = sizes[i]
			const x = BLOCK_SIZE * colliderPos.x + BLOCK_SIZE/2 - colliderSize.width/2;
			const y = BLOCK_SIZE * colliderPos.y + BLOCK_SIZE/2 - colliderSize.height/2
			const box2 = {
				minX: x,
				minY: y,
				maxX: x + colliderSize.width,
				maxY: y + colliderSize.height,
			}

			if (aabb(box1, box2)) {
				const resume = resolver(world, bomber, bComponents, entities[i], box1, box2)
				if (!resume) {
					return
				}
			}
		}
	}
}

interface Box {
	minX: number,
	minY: number,
	maxX: number,
	maxY: number,
}

function aabb(a: Box, b: Box): Boolean {
	return (
		a.minX < b.maxX &&
		a.maxX > b.minX &&
		a.minY < b.maxY &&
		a.maxY > b.minY
	);
}

function resolveAABB(a: Box, b: Box, threshold: number): { x: number, y: number } {
	const lenLeft = Math.abs(b.minX - a.maxX)
	const lenRight = Math.abs(b.maxX - a.minX)
	const lenUp = Math.abs(b.minY - a.maxY)
	const lenDown = Math.abs(b.maxY - a.minY)

	const left = {len: lenLeft, vec: {x: lenLeft, y: 0}}
	const right = {len: lenRight, vec: {x: -lenRight, y: 0}}
	const up = {len: lenUp, vec: {x: 0, y: lenUp}}
	const down = {len: lenDown, vec: {x: 0, y: -lenDown}}

	const arr = [left, right, up, down]
	arr.sort(({len: a}, {len: b}) => a - b)

	let penetration = arr[0].vec

	// Ignore tiny corner collisions
	// console.log(arr[0].v, arr[1].v)
	if (arr[1].len <= threshold) {
		penetration = {x: 0, y: 0}
	}

	return penetration
}

function integratePositionsWithDamage(world: World<TickData>, bomberEID: number, res: BomberQueryResult, _collider: number, a: Box, b: Box) {
	if (!integrateInner(world, res, a, b)) {
		// If overlapping, but penetrating enough to not be pushed outside
		if (!world.has(bomberEID, IFrame)) {
			applyDamage(world, bomberEID, res[1], res[0])
		}
	}

	return true
}

function integratePositions(world: World<TickData>, _: number, res: BomberQueryResult, _collider: number, a: Box, b: Box) {
	integrateInner(world, res, a, b)

	return true
}


function integrateInner(world: World<TickData>,  [, position, bomber]: BomberQueryResult, a: Box, b: Box) {
	const threshold = observe(bomber).speed * world.latestTickData.dt / 1000 + 0.1
	const penetration = resolveAABB(a, b, threshold)

	if (Math.abs(penetration.x) <= threshold * 2 &&
		Math.abs(penetration.y) <= threshold * 2) {
		const pos = observe(position)
		pos.x -= penetration.x
		pos.y -= penetration.y
		return true
	}

	return false
}

function pickUpPowerUp(world: World, bomberEID: number, [,, bomber]: BomberQueryResult, powerup: number, _a: Box, _b: Box) {
	const powerUp = world.get(powerup, PowerUp)
	switch (powerUp.type) {
		case 0:
			observe(bomber).power++

			break;
		case 1:
			observe(bomber).bombCount++

			break;
		case 2:
			observe(bomber).speed += 30

			break;
		case 3:
			// Kick

			break;
		case 4:
			// Throw

			break;
		default:
			break;
	}

	world.destroy(powerup)

	const socket = world.tryGet(bomberEID, Connection) as Socket
	if (socket) {
		socket.emit("play-ding")
	}

	return true
}

function explosionDamage(world: World<TickData>, bomber: number, [bomberSize, bomberPos]: BomberQueryResult, _explosion: number, _a: Box, _b: Box) {
	applyDamage(world, bomber, bomberPos, bomberSize)

	return false
}

function applyDamage(world: World<TickData>, bomber: number, bomberPos: {x: number, y: number}, bomberSize: {width: number, height: number}) {
	const lives = world.get(bomber, Lives)

	observe(lives).current -= 1

	if (lives.current > 0) {
		addTimer(world, bomber, 3)
		world.attach(bomber, component(IFrame))
	} else {
		world.detach(bomber, Lives, Velocity)
	}

	// Adding the ghost
	const ghost = world.create(
		component(Ghost),
		cloneComponent(Position, bomberPos),
		cloneComponent(Size, bomberSize),
	)

	addTimer(world, ghost, 1)
}
