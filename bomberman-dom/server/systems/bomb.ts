import {component, createQuery, observe, useMonitor, World} from "@javelin/ecs";
import {
	Bomb, Bomber,
	BombPrimed,
	GridPosition,
	Owner,
	Team,
	TimerFinished
} from "../schemas";
import {addTimer} from "./timer";
import {TickData} from "../net";
import {useCaches} from "../caches";
import {spawnExplosion} from "../scenes";

const toPrime = createQuery(Bomb, TimerFinished).not(BombPrimed)
const toExplode = createQuery(Bomb, GridPosition, Team, TimerFinished, BombPrimed)

const exploded = createQuery(Bomb, Owner)

export const bombSystem = (world: World<TickData>) => {
	toPrime((eid, []) => {
		world.detach(eid, TimerFinished)

		addTimer(world, eid, 0.5)

		world.attach(eid, component(BombPrimed))
	})

	const caches = useCaches()

	toExplode((eid, [bomb, gridPos, team]) => {
		const x = gridPos.x
		const y = gridPos.y

		const power = bomb.power

		spawnExplosion(world, caches, x, y, power, team)

		world.destroy(eid)
	})

	useMonitor(exploded, undefined, (e, [, owner]) => {
		const bomber = world.get(owner.eid, Bomber)
		observe(bomber).bombCount += 1
	})

	return world
}
