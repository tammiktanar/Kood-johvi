import {component, createQuery, World} from "@javelin/ecs";
import {Ghost, IFrame, Timer, TimerFinished} from "../schemas";
import {TickData} from "../net";

const timerQuery = createQuery(Timer)
const iframes = createQuery(TimerFinished, IFrame)
const ghosts = createQuery(TimerFinished, Ghost)
export function timerSystem(world: World<TickData>) {
	const timersDone = Array<number>()
	timerQuery((e, [{end}]) => {
		if (end <= world.latestTickData.now) {
			timersDone.push(e)
			// 4: Timer schema
			world.detachImmediate(e, [4])
			world.attachImmediate(e, [component(TimerFinished)])
		}
	})

	iframes((e) => {
		world.detach(e, TimerFinished, IFrame)
	})

	ghosts((e) => {
		world.destroy(e)
	})
}

export function addTimer(world: World<TickData>, entity: number, duration: number) {
	const now = world.latestTickData.now
	world.attach(entity, component(Timer, {start: now, end: now + duration * 1000}))
}
