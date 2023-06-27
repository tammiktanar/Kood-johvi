import {createQuery, observe, useMonitor, World} from "@javelin/ecs";
import {Lives, Position, Velocity, VelocityGoal} from "../schemas";
import {approach} from "../lib";
import {PLAYER_ACCELERATION} from "../env";
import {TickData} from "../net";

const moving = createQuery(Velocity, Position).not(VelocityGoal)
const players = createQuery(Velocity, Position, VelocityGoal)

export function physicsSystem(world: World<TickData>) {
	moving((_, [vel, position]) => {
		const dt = world.latestTickData.dt / 1000

		const pos = observe(position)
		pos.x += vel.x * dt
		pos.y += vel.y * dt
	})

	players((_, [velocity, position, velGoal]) => {
		const dt = world.latestTickData.dt / 1000

		const speed = Math.sqrt(velGoal.speed)
		const vel = observe(velocity)
		vel.x = approach(velGoal.x, vel.x, PLAYER_ACCELERATION * speed * dt)
		vel.y = approach(velGoal.y, vel.y, PLAYER_ACCELERATION * speed * dt)

		const pos = observe(position)
		pos.x += vel.x * dt
		pos.y += vel.y * dt
	})
}
