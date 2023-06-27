import {createEffect, createQuery} from "@javelin/ecs";
import {GameElement, Grid} from "./schemas";

export let ROOT = document.querySelector("#app") as HTMLDivElement
export let CHAT_INPUT = document.querySelector("#chat-input") as HTMLInputElement
export let CHAT_MESSAGES = document.querySelector("#chat-messages") as HTMLDivElement

export let USERNAME = localStorage.getItem('player_name') || "";
export let USE_COLOR = localStorage.getItem('player_color') || "#"+Math.floor(Math.random()*16777215).toString(16);
export let MAX_TIME_LENGTH = 180

export let production = Boolean(import.meta.env.PRODUCTION)
export let websocket_port = import.meta.env.PORT || 8000


// START Scaling stuff
let scale = Number(localStorage.getItem('scaling')) || 2
export let SCALING = 1

function updateScaling() {
	const pixelMult = devicePixelRatio || 1

	SCALING = 1 / pixelMult * scale

	document.body.style.setProperty("--scaling", SCALING.toString())
}

window.addEventListener("resize", () => updateScaling())
updateScaling()

const smaller = document.querySelector("#scaling-down") as HTMLElement
smaller.addEventListener("click", () => changeScale(-1))
const bigger = document.querySelector("#scaling-up") as HTMLElement
bigger.addEventListener("click", () => changeScale(1))

function changeScale(change: number) {
	scale += change
	scale = Math.max(1, Math.min(scale, 8))

	updateScaling()
	localStorage.setItem("scaling", scale.toString())
}
// END Scaling stuff

const gridQuery = createQuery(Grid, GameElement)
export const useGrid = createEffect<HTMLElement | undefined>(() => {
	let grid: HTMLElement | undefined = undefined

	return () => {
		grid = undefined
		gridQuery((_, [, elem]) => grid = elem as HTMLElement)
		return grid
	}
}, {shared: true})


/////////////////
// AUDIO STUFF //
/////////////////

