// noinspection DuplicatedCode

import { component, toComponent, World } from "@javelin/ecs"
import {ROOT} from "../globals";
import {socket} from "../net";
import { GameElement, KeepElement } from "../schemas";
import {resetWorld, world} from "../world";
import { addSoundFXButton, addSoundMusicButton } from "./game-scene";
import {createDingButton} from "../lib";
import {playMusic} from "../audio";
import {createLobbyScreen} from "./lobby";


export function createMenuScene(world: World) {
	playMusic("menu")

	console.log("Opening main menu")
	ROOT.dataset.scene = "main-menu"

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

    let start_game_quick = createDingButton() // Making start game button
    start_game_quick.classList.add("start-game")
    start_game_quick.classList.add("image-button")
    start_game_quick.onclick = function(){quickPlay(world)};
	start_game_quick.innerText = "Quick play"
	main_menu.appendChild(start_game_quick)
	world.create(
		component(KeepElement),
		toComponent(start_game_quick, GameElement)
	)


	let lobby_code = document.createElement("input") // Making username input field
	lobby_code.classList.add("lobby-code")
	lobby_code.maxLength = 16
	lobby_code.placeholder = "Lobby code"

	main_menu.appendChild(lobby_code)
	world.create(
		component(KeepElement),
		toComponent(lobby_code, GameElement)
	)

	let join_lobby_code = createDingButton() // Making create lobby button
    join_lobby_code.classList.add("start-game")
    join_lobby_code.classList.add("image-button")
    join_lobby_code.onclick = function(){joinLobby(lobby_code.value)};
	join_lobby_code.innerText = "Join lobby"
	main_menu.appendChild(join_lobby_code)
	world.create(
		component(KeepElement),
		toComponent(join_lobby_code, GameElement)
	)

	let create_lobby = createDingButton() // Making create lobby button
    create_lobby.classList.add("start-game")
    create_lobby.classList.add("image-button")
    create_lobby.onclick = function(){createLobby()};
	create_lobby.innerText = "Create lobby"
	main_menu.appendChild(create_lobby)
	world.create(
		component(KeepElement),
		toComponent(create_lobby, GameElement)
	)

	let singleplayer = createDingButton() // Making create lobby button
	singleplayer.id = "old-ver-button"
	singleplayer.classList.add("start-game")
	singleplayer.classList.add("image-button")
	singleplayer.onclick = function(){window.location.href = "https://old-bomberman.olari.dev/";};
	singleplayer.innerText = "Old version"
	main_menu.appendChild(singleplayer)
	world.create(
		component(KeepElement),
		toComponent(singleplayer, GameElement)
	)

	ROOT.append(main_menu)
}

function quickPlay(_world: World) {
	socket.emit("quick-play", () => {
		console.log('Quick play')

		resetWorld(() => {
			createLobbyScreen(world)
		})
	})
}

function joinLobby(id: string) {
	if (Number(id)) {
		console.log(`Joining lobby via code: ${id}`)

		socket.emit("join-lobby", Number(id), (bool: boolean) => {
				if (bool){
					console.log('Joined lobby');
					resetWorld(() => {
						createLobbyScreen(world)
					})
				} else {
					alert(`No lobby with code ${id}`)
				}
			}
		)
	} else {
		alert(`${id} is not a valid lobby code`)
	}
}

function createLobby() {
	socket.emit("create-lobby", () => {
		resetWorld(() => {
			createLobbyScreen(world)
		})
	})
}
