import {
	createQuery,
	Query,
	Selector,
	SelectorResult,
	toComponent, useRef,
	World
} from "@javelin/ecs";
import { getFilter } from "../filters";
import {useGrid} from "../globals";
import {
	Bomb,
	Bomber,
	BombPrimed,
	Crate, Explosion,
	GameElement, Ghost, GrayWall, Hidden, PowerUp,
	Team,
	Wall
} from "../schemas";
import {playFX} from "../audio";

const walls = createQuery(Wall).not(GameElement)
const grayWalls = createQuery(GrayWall).not(GameElement, Hidden)
const crates = createQuery(Crate).not(GameElement)
const bombers = createQuery(Bomber, Team).not(GameElement)
const bombs = createQuery(Bomb, Team).not(GameElement)
const bombsPrimed = createQuery(GameElement, Bomb, Team, BombPrimed)
const explosions = createQuery(Explosion, Team).not(GameElement)
const ghosts = createQuery(Ghost).not(GameElement)
const powerups = createQuery(PowerUp).not(GameElement)

export function addElementsSystem(world: World) {
	const grid = useGrid()

	if (grid) {
		addElement(world, grid, walls, (elem) => {
			elem.classList.add("wall-block")
		})

		addElement(world, grid, grayWalls, (elem) => {
			elem.classList.add("gray-wall-block")
		})

		addElement(world, grid, crates, (elem) => {
			elem.classList.add("crate-block")
		})

		addElement(world, grid, bombers, (elem, eid, [_, team]) => {
			let sprite = document.createElement("div")
			sprite.classList.add("sprite")

			let overlay = document.createElement("div")
			overlay.classList.add("overlay")

			sprite.appendChild(overlay)

			elem.appendChild(sprite)
			elem.classList.add("bomber")

			if (team.color != "") {
				overlay.style.filter = getFilter("overlay-multiply", team.color)
			}
		})

		addElement(world, grid, bombs, (elem, bomberID,[_, team]) => {
			let overlay = document.createElement("div")
			overlay.classList.add("overlay")

			elem.appendChild(overlay)
			elem.classList.add("bomb")

			if (team.color != "") {
				overlay.style.filter = getFilter("overlay-multiply", team.color)
			}
		})

		bombsPrimed((eid, [elem]) => {
			(elem as HTMLElement).classList.add("primed")
		})

		let lastExplosionTime = useRef(0)
		addElement(world, grid, explosions, (elem, eid, [explosion, team]) => {
			let sprite = document.createElement("div")
			sprite.classList.add("sprite")
			elem.appendChild(sprite)

			let type = ""
			switch (explosion.type) {
				case 0:
					type = "cross"
					// Play explosion sound + throttle repeated sounds
					const now = performance.now()
					if (lastExplosionTime.value < now-40) {
						playFX("explosion")
						lastExplosionTime.value = now
					}
					break
				case 1: type = "line"; break
				case 2: type = "end"; break
			}

			let direction = ""
			switch (explosion.direction) {
				case 0: direction = "up"; break
				case 1: direction = "right"; break
				case 2: direction = "down"; break
				case 3: direction = "left"; break
			}

			elem.classList.add("explosion")
			elem.classList.add(type)
			elem.classList.add(direction)

			if (team.color != "") {
				sprite.style.filter = getFilter("overlay", team.color)
			}
		})

		addElement(world, grid, ghosts, (elem) => {
			let sprite = document.createElement("div")
			sprite.classList.add("sprite")
			elem.appendChild(sprite)

			elem.classList.add("ghost")
		})

		addElement(world, grid, powerups, (elem, eid, [powerup]) => {
			let type = ""
			switch (powerup.type) {
				case 0: type = "thunder-power-up"; break;
				case 1: type = "bomb-power-up"; break;
				case 2: type = "speed-power-up"; break;
				case 3: type = "kick-power-up"; break;
				case 4: type = "throw-power-up"; break;
			}

			elem.classList.add("power-up")
			elem.classList.add(type)
		})
	}
}

interface ElementCallback<T extends Selector> {
	(elem: HTMLElement, entity: number, components: SelectorResult<T>): void
}

function addElement<T extends Selector>(world: World, appendTo: HTMLElement, query: Query<T>, callback: ElementCallback<T>) {
	query((ent, components) => {
		const elem = document.createElement("div")

		callback(elem, ent, components)

		appendTo.append(elem)

		world.attachImmediate(ent, [toComponent(elem, GameElement)])
	})
}
