import {component, toComponent} from "@javelin/ecs";
import {GameElement, KeepElement} from "../schemas";
import {world} from "../world";
import {getFilter} from "../filters";

export function displayWinner(winner: {team: number, color: string, name: string}) {
	const grid = document.querySelector("#grid")
	if (!grid) return

	const winner_overlay = document.createElement("div")
	winner_overlay.id = "winner-overlay"

	const winner_text = document.createElement("div")
	winner_text.id = "winner-text"
	winner_overlay.append(winner_text)

	if (winner.team !== 0) {
		const winner_name = document.createElement("span")
		winner_name.id = "winner-name"
		winner_name.innerText = winner.name
		winner_name.style.color = winner.color
		winner_text.append(winner_name)
	}

	const other_text = document.createElement("span")
	other_text.innerText = winner.team !== 0 ? " is the winner!" : "It's a tie!"
	winner_text.append(other_text)

	const player_div = createPlayerElement({
		username: winner.name,
		color: winner.color,
		ready: true,
	}, false)
	winner_overlay.append(player_div)
	player_div.style.setProperty("--player-width", "calc(116px * var(--scaling))")

	grid.append(winner_overlay)

	world.create(
		component(KeepElement),
		toComponent(winner_overlay, GameElement)
	)
}

export function createPlayerElement(player: {username: string, color: string, ready: boolean}, showName: boolean) {
	let player_div = document.createElement("div")

	player_div.classList.add("player")

	if (showName) {
		let player_name_div = document.createElement("div")
		player_name_div.classList.add("player-name")
		player_name_div.innerText = player.username
		player_div.appendChild(player_name_div)
	}

	let player_skin_div = document.createElement("div")
	let player_clothes_div = document.createElement("div")

	player_clothes_div.classList.add("player-clothes")
	player_skin_div.classList.add("player-skin")

	player_clothes_div.style.filter = getFilter("overlay-multiply", player.color)


	let readyState =  player.ready ? "happy" : "sad"

	player_clothes_div.classList.add(readyState)
	player_skin_div.classList.add(readyState)


	player_skin_div.appendChild(player_clothes_div)
	player_div.appendChild(player_skin_div)

	return player_div
}
