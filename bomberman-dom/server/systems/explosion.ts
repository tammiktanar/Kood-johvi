import {createQuery, World} from "@javelin/ecs";
import {Crate, Explosion, GridPosition, TimerFinished} from "../schemas";
import {spawnPowerUp} from "../scenes";
import {POWERUP_CHANCE} from "../env";

const explosionDestroy = createQuery(Explosion, TimerFinished)
const crateDestroy = createQuery(Crate, GridPosition, TimerFinished)
export const explosionSystem = (world: World) => {
	explosionDestroy((eid) => {
		world.destroy(eid)
	})


	crateDestroy((eid, [, gridPos]) => {
		world.destroy(eid)

		if (Math.floor(Math.random() * 100) < POWERUP_CHANCE) {
			spawnPowerUp(world, gridPos)
		}
	})

	return world
}
