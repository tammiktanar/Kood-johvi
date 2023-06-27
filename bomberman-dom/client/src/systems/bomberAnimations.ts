import { createQuery, useMonitor, World } from "@javelin/ecs"
import { Bomber, GameElement, IFrame, Lives, Velocity } from "../schemas"

const bomberQuery = createQuery(GameElement, Velocity, Bomber, Lives)

const iframes = createQuery(GameElement, Bomber, IFrame)

const deadEnter = createQuery(GameElement, Bomber).not(Lives)

export const bomberAnimationSystem = (_world: World) => {
	bomberQuery((eid, [elem, velocity] ) => {
		const vel = {x: velocity.x, y: velocity.y}

		let direction
		if (vel.x !== 0 || vel.y !== 0) {
			if (Math.abs(vel.x) >= Math.abs(vel.y)) {
				// Horizontal
				direction = vel.x > 0 ? "right" : "left"

			} else {
				// Vertical
				direction = vel.y > 0 ? "down" : "up"
			}
		}

		const e = elem as HTMLDivElement
		if (direction) {
			e.dataset.direction = direction
		}
		e.classList.toggle("moving", vel.x != 0 || vel.y != 0)
	})

	useMonitor(bomberQuery, undefined, (_, [element]) => {
		const elem = element as HTMLElement
		elem.classList.remove("moving")
		elem.dataset.direction = ""
	})

	useMonitor(iframes, (eid, [elem] ) => {
		const e = elem as HTMLDivElement
		e.classList.add("iframe")
	}, (eid, [elem]) => {
		const e = elem as HTMLDivElement
		e.classList.remove("iframe")
	})

	useMonitor(deadEnter, (eid, [elem] ) => {
		const e = elem as HTMLDivElement
		e.classList.add("dead")
	})

}
