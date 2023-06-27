// noinspection DuplicatedCode

import { component, toComponent, World } from "@javelin/ecs";
import { ROOT } from "../globals";
import {socket} from "../net";
import { GameElement, KeepElement } from "../schemas";
import { addSoundFXButton, addSoundMusicButton } from "./game-scene";
import {resetWorld} from "../world";
import {getFilter} from "../filters";
import {createDingButton} from "../lib";
import {createMenuScene} from "./main-menu";
import {playMusic} from "../audio";
import {createPlayerElement} from "./winner";

let player_list_elem = document.createElement("div") as HTMLDivElement
let lobby_text_elem = document.createElement("a") as HTMLAnchorElement
let lobby_timer_text_elem = document.createElement("a") as HTMLAnchorElement
let ready_up_button_elem = document.createElement("button") as HTMLButtonElement

let start_lobby_performance_time = performance.now()
let lobby_ready_state_time = 20;
let lobby_game_ready_state_time = 10;
let interval_state = ""
export let lobby_interval: any

export function createLobbyScreen(world: World) {
	playMusic("menu")

	console.log("Creating lobby screen")
	ROOT.dataset.scene = "lobby-scene"



	const main_menu = document.createElement("div") // Making main menu div
	main_menu.id = "main-menu"
	main_menu.classList.add("flex-container")
	world.create(
		component(KeepElement),
		toComponent(main_menu, GameElement)
	)

	let sound_div = document.createElement("div")
	sound_div.classList.add("sound-div")
	addSoundMusicButton(world, sound_div)
	addSoundFXButton(world, sound_div)
	main_menu.appendChild(sound_div)
	world.create(
		component(KeepElement),
		toComponent(sound_div, GameElement)
	)

	const lobby_text = document.createElement("a")
	lobby_text.innerText = "Lobby"
	lobby_text.classList.add("lobby-text")
	main_menu.appendChild(lobby_text)
	lobby_text_elem = lobby_text
	world.create(
		component(KeepElement),
		toComponent(lobby_text, GameElement)
	)

	const lobby_timer_text = document.createElement("a")
	lobby_timer_text.innerText = ""
	lobby_timer_text.classList.add("lobby-timer-text")
	main_menu.appendChild(lobby_timer_text)
	lobby_timer_text_elem = lobby_timer_text
	world.create(
		component(KeepElement),
		toComponent(lobby_timer_text, GameElement)
	)

    let ready_up_button = createDingButton() // Making start game button
    ready_up_button.classList.add("start-game")
    ready_up_button.classList.add("image-button")
    ready_up_button.onclick = function(){readyUp(ready_up_button)};
	ready_up_button.innerText = "Ready up"
	ready_up_button_elem = ready_up_button
	main_menu.appendChild(ready_up_button)
	world.create(
		component(KeepElement),
		toComponent(ready_up_button, GameElement)
	)

	let leave_lobby = createDingButton() // Making start game button
    leave_lobby.classList.add("start-game")
    leave_lobby.classList.add("image-button")
    leave_lobby.onclick = function(){
		leaveLobby(world)
	};
	leave_lobby.innerText = "Leave lobby"
	main_menu.appendChild(leave_lobby)
	world.create(
		component(KeepElement),
		toComponent(leave_lobby, GameElement)
	)

	const player_list = document.createElement("div") // Making main menu div
	player_list.id = "player_list"
	player_list.classList.add("player-list")
	main_menu.appendChild(player_list)
	player_list_elem = player_list

	world.create(
		component(KeepElement),
		toComponent(player_list, GameElement)
	)



	ROOT.append(main_menu)

	socket.emit('get-lobby-state', (state: any) => {
		console.log("get lobby state:", state)
		updateLobbyViaState(state)
		startSetReadyTimer()
	})
}

export function updateLobbyViaState(state: any) {
	if (ROOT.dataset.scene !== "lobby-scene") return

	updateLobbyCode(state.id)
	updatePlayerList(state.members)

	if (state.countdown) {
		startGameReadyTimer()
	} else if (interval_state == "game_ready") {
		clearInterval(lobby_interval)
		updateTimerState("")
	}
}

export function updateTimerState(text: string) {
	lobby_timer_text_elem.innerHTML = text
}

export function updateLobbyCode(code: string) {
	lobby_text_elem.innerText = `Lobby\nLobby code: ${code}📋`
	lobby_text_elem.title = "Copy to clipboard"
	lobby_text_elem.onclick = function(){navigator.clipboard.writeText(code);}
}

export function updatePlayerList(players: Array<{username: string, color: string, ready: boolean}>) {
	player_list_elem.innerHTML = ""

	players.forEach(player => {
		const player_div = createPlayerElement(player, true)
		player_list_elem.append(player_div)
	});
}















function updateLobbyTimers(type: string) {
	let cur_time = performance.now()

	let difference = cur_time - start_lobby_performance_time
	let seconds = 0
	let timer_text = ""

	switch (type) {
		case "set_ready":seconds = lobby_ready_state_time - Math.round(difference / 1000); timer_text= `You'll be readied in: ${seconds}`; break;
		case "game_ready":seconds = lobby_game_ready_state_time - Math.round(difference / 1000); timer_text= `Game starts in: ${seconds}`; break;
		default:return;
	}


	if (seconds <= 0){
		clearInterval(lobby_interval)

		switch (type) {
			case "set_ready": ready_up_button_elem.click(); break;
			case "game_ready":break;

			default:
				break;
		}

		timer_text = ""
	}

	updateTimerState(timer_text)
}

export function startSetReadyTimer() {
	clearInterval(lobby_interval)

	interval_state = "set_ready"
	start_lobby_performance_time = performance.now()
	updateLobbyTimers("set_ready")
	lobby_interval = setInterval(updateLobbyTimers, 500, "set_ready");
}

export function startGameReadyTimer() {
	clearInterval(lobby_interval)

	interval_state = "game_ready"
	start_lobby_performance_time = performance.now()
	updateLobbyTimers("game_ready")
	lobby_interval = setInterval(updateLobbyTimers, 500, "game_ready");
}















///////////////////
// NETWORK STUFF //
///////////////////

socket.on("update-lobby-state", (state) => {
	updateLobbyViaState(state)
})

function readyUp(elem: HTMLButtonElement){
	socket.emit('ready')

	elem.innerText = "Unready"
	elem.onclick = function(){unReadyUp(elem)};

	clearInterval(lobby_interval)
	updateTimerState("")
}

function unReadyUp(elem: HTMLButtonElement){
	socket.emit('unready')

	elem.innerText = "Ready up"
	elem.onclick = function(){readyUp(elem)};

	startSetReadyTimer()
}


function leaveLobby(world: World) {
	socket.emit('leave-lobby', () => {
		resetWorld(() => {
			createMenuScene(world)
		})
	})
}
