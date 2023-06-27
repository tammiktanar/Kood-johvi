import {createQuery, useMonitor, World} from "@javelin/ecs";
import {GameElement, GridPosition, KeepElement, Position, Size} from "../schemas";
import {SCALING} from "../globals";

const renderables = createQuery(GameElement, Position, Size)
const gridRenderables = createQuery(GameElement, GridPosition, Size)
const toRemove = createQuery(GameElement).not(GridPosition, Position, Size, KeepElement)

export function renderSystem(world: World) {
	renderables((e, [element, pos, size ]) => {
		let elem = element as HTMLElement

		elem.style.left = pos.x * SCALING + "px"
		elem.style.top = pos.y * SCALING + "px"

		elem.style.width = size.width * SCALING + "px"
		elem.style.height = size.height * SCALING + "px"
	})

	useMonitor(gridRenderables,(e, [element, gridPos, size ]) => {
		let elem = element as HTMLElement

		elem.style.left = `calc(${30 * gridPos.x + 30/2 - size.width/2}px * var(--scaling))`
		elem.style.top = `calc(${30 * gridPos.y + 30/2 - size.height/2}px * var(--scaling))`

		elem.style.width = `calc(${size.width}px * var(--scaling))`
		elem.style.height = `calc(${size.height}px * var(--scaling))`
	})

	toRemove((ent, [elem]) => {
		(elem as HTMLElement).remove()
		world.destroyImmediate(ent)
	})
}
