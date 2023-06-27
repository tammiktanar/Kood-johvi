import {createQuery, World} from "@javelin/ecs";
import {Bomber, Input, Lives, Velocity, VelocityGoal} from "../schemas";
import {spawnBomb} from "../scenes";
import {TickData} from "../net";
import {useBombCache, useCaches} from "../caches";
import {PLAYER_MOVEMENT_SLOPE} from "../env";

const playerQuery = createQuery(Input, Bomber, VelocityGoal, Lives)
export function inputSystem(world: World<TickData>) {
	const caches = useCaches()

	playerQuery((e, [input, bomber, velGoal]) => {
		let speed = input.walk ? 50 : bomber.speed
		speed = Math.log1p(speed / PLAYER_MOVEMENT_SLOPE) * PLAYER_MOVEMENT_SLOPE
		velGoal.x = velX(input) * speed
		velGoal.y = velY(input) * speed
		velGoal.speed = speed

		if (input.bomb && bomber.bombCount > 0) {
			input.bomb = false
			spawnBomb(world, caches, e)
		}
	})


	return world
}

function velX(input: InputI): number {
	let v = 0
	if (input.right) v += 1
	if (input.left) v -= 1
	return v
}

function velY(input: InputI): number {
	let v = 0
	if (input.down) v += 1
	if (input.up) v -= 1
	return v
}

interface InputI {
	up: boolean,
	down: boolean,
	left: boolean,
	right: boolean,
	bomb: boolean,
	walk: boolean,
}
