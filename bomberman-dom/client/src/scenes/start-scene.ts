import { component, toComponent, World } from "@javelin/ecs";
import { ROOT, USE_COLOR, USERNAME } from "../globals";
import {socket} from "../net";
import { GameElement, KeepElement } from "../schemas";
import { resetWorld } from "../world";
import { createMenuScene } from "./main-menu";
import {initAudio, playFX} from "../audio";
import { createChat, createStartSceneChat } from "../chat";

export function createStartScreen(world: World) {
	console.log("Opening start screen")
	ROOT.dataset.scene = "start-scene"


	const main_menu = document.createElement("div") // Making main menu div
	main_menu.id = "main-menu"
	main_menu.classList.add("flex-container")
	world.create(
		component(KeepElement),
		toComponent(main_menu, GameElement)
	)



    let start_game = document.createElement("button") // Making start game button
	start_game.tabIndex = -1
    start_game.classList.add("start-game")
    start_game.classList.add("image-button")
	start_game.innerText = "Start game"
	main_menu.appendChild(start_game)
	world.create(
		component(KeepElement),
		toComponent(start_game, GameElement)
	)


	let user_name_field = document.createElement("input") // Making username input field
	user_name_field.classList.add("user-name-field")
	user_name_field.maxLength = 16
	user_name_field.placeholder = "Enter username"
	user_name_field.value = USERNAME
	main_menu.appendChild(user_name_field)
	world.create(
		component(KeepElement),
		toComponent(user_name_field, GameElement)
	)

	let color_code = document.createElement("input") // Making username input field
	color_code.classList.add("color-code")
	color_code.type = "color"
	color_code.id = "color-code"
	color_code.placeholder = "Enter lobby code"
	color_code.value = USE_COLOR


	main_menu.appendChild(color_code)
	world.create(
		component(KeepElement),
		toComponent(color_code, GameElement)
	)




    start_game.onclick = function(){
		initAudio()
		playFX("ding")
		openMainScreen(world, user_name_field, color_code)
	};

	createStartSceneChat()

	ROOT.append(main_menu)
}




function openMainScreen(world: World, user_name_field: HTMLInputElement, color_code: HTMLInputElement) {
	if (!socket.connected) {
		alert("Connection error")
		return
	}

	user_name_field.value = user_name_field.value.trim()
	color_code.value = color_code.value.trim()

	if (user_name_field.value == "" || color_code.value == "" ) {
		alert("Please enter a username")

		return
	} else if (user_name_field.value.length > 16) {
		alert("Please have a username with a max length of 16 characters")

		return
	}

	localStorage.setItem('player_name', user_name_field.value)
	localStorage.setItem('player_color', color_code.value)

	socket.emit('set-username', user_name_field.value)
	socket.emit('set-color', color_code.value)

	createChat()

	resetWorld(() => {
		createMenuScene(world)
	})
}
