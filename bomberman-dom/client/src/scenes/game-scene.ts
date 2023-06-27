import { component, string, toComponent, World } from "@javelin/ecs"
import { MAX_TIME_LENGTH, ROOT } from "../globals"
import {socket} from "../net"
import {GameElement, Grid, KeepElement, Position, Size} from "../schemas"
import { createMenuScene } from "./main-menu"
import {resetWorld} from "../world";
import { getFilter } from "../filters"
import {
	FX_MUTED,
	MUSIC_MUTED,
	playMusic,
	toggleMuteFX,
	toggleMuteMusic
} from "../audio";
import {createDingButton} from "../lib";
import { lobby_interval } from "./lobby"

let lives_1: HTMLDivElement
let lives_2: HTMLDivElement
let lives_3: HTMLDivElement
let lives_4: HTMLDivElement

let portrait_1: HTMLDivElement
let portrait_2: HTMLDivElement
let portrait_3: HTMLDivElement
let portrait_4: HTMLDivElement

let time_number_elem: HTMLDivElement
let time_bar_elem: HTMLProgressElement

let start_performance_time = performance.now();
export let timerInterval: any


export function loadGameScene(world: World) {
	console.log("Opening game scene")
	ROOT.dataset.scene = "game-scene"


	const grid_elem = document.createElement("div")
	grid_elem.id = "grid"
	world.create(
		component(Grid),
		component(Position, {x: 92, y: 9}),
		component(Size, {width: 450, height: 390}),
		toComponent(grid_elem, GameElement),
	)
	ROOT.append(grid_elem)


	const side_bar = document.createElement("div")
	side_bar.id = "sidebar"
	world.create(
		component(KeepElement),
		toComponent(side_bar, GameElement)
	)
	ROOT.append(side_bar)


	let quit_game = createDingButton()
	quit_game.classList.add("quit-game")
	quit_game.classList.add("image-button")
	quit_game.onclick = function(){
		leaveGame(world)
	};

	side_bar.appendChild(quit_game)
	world.create(
		component(KeepElement),
		toComponent(quit_game, GameElement)
	)

	addSoundFXButton(world, side_bar)
	addSoundMusicButton(world, side_bar)
	addTimerAndBar(world, side_bar, MAX_TIME_LENGTH)


	lives_1 = addLives(world, side_bar, "1", 3)
	lives_2 = addLives(world, side_bar, "2", 3)
	lives_3 = addLives(world, side_bar, "3", 3)
	lives_4 = addLives(world, side_bar, "4", 3)

	portrait_1 = addPlayerPortrait(world, side_bar, "1")
	portrait_2 = addPlayerPortrait(world, side_bar, "2")
	portrait_3 = addPlayerPortrait(world, side_bar, "3")
	portrait_4 = addPlayerPortrait(world, side_bar, "4")

	updateTimer(MAX_TIME_LENGTH)
	updateTimerBar(MAX_TIME_LENGTH)

	setPlayerPortraitDisabled(3)
	setPlayerPortraitDisabled(4)

	playMusic("game")

	ROOT.focus()
}

export function addSoundFXButton(world: World, side_bar: HTMLDivElement) {
	let fx_button = createDingButton()
	fx_button.id = "fx-button"
	fx_button.classList.add("fx-button")
	fx_button.classList.add("image-button")
	fx_button.dataset.fx = String(!FX_MUTED)
	fx_button.onclick = function(){onClickFx(fx_button)};

	side_bar.appendChild(fx_button)
	world.create(
		component(KeepElement),
		toComponent(fx_button, GameElement)
	)
}

export function addSoundMusicButton(world: World, side_bar: HTMLDivElement) {
	let music_button = createDingButton()
	music_button.id = "music-button"
	music_button.classList.add("music-button")
	music_button.classList.add("image-button")
	music_button.dataset.music = String(!MUSIC_MUTED)
	music_button.onclick = function(){onClickMusic(music_button)};


	side_bar.appendChild(music_button)
	world.create(
		component(KeepElement),
		toComponent(music_button, GameElement)
	)
}

function addLives(world: World, side_bar: HTMLDivElement, team: string, lives: number) {
	let lives_div = document.createElement("div")
	lives_div.id = `lives-${team}`
	lives_div.classList.add("lives")
	lives_div.classList.add(`p-${team}`)
	lives_div.innerText = String(lives)

	side_bar.appendChild(lives_div)
	world.create(
		component(KeepElement),
		toComponent(lives_div, GameElement)
	)

	return lives_div
}

export function updateLives(team: number, amount: number) {
	if (!lives_1) {return;}
	if (!lives_2) {return;}
	if (!lives_3) {return;}
	if (!lives_4) {return;}


	switch (team) {
		case 1: lives_1.innerText = String(amount); break;
		case 2: lives_2.innerText = String(amount); break;
		case 3: lives_3.innerText = String(amount); break;
		case 4: lives_4.innerText = String(amount); break;
	}
}



function addTimerAndBar(world: World, side_bar: HTMLDivElement, max: number) {
	let timer = document.createElement("div")
	time_number_elem = timer
	timer.id = "timer"
	timer.classList.add("timer")

	side_bar.appendChild(timer)
	world.create(
		component(KeepElement),
		toComponent(timer, GameElement)
	)

	let time_bar = document.createElement("progress")
	time_bar_elem = time_bar
	time_bar.id = "timerBar"
	time_bar.classList.add("timerBar")
	time_bar.max = max
	time_bar.value = max / 2

	side_bar.appendChild(time_bar)
	world.create(
		component(KeepElement),
		toComponent(time_bar, GameElement)
	)
}


function addPlayerPortrait(world: World, side_bar: HTMLDivElement, team: string) {

	let portrait_div = document.createElement("div")
	portrait_div.id = `portrait-${team}`

	portrait_div.dataset.type = "clothes"
	portrait_div.classList.add("portrait")
	portrait_div.classList.add(`p-${team}`)

	side_bar.appendChild(portrait_div)
	world.create(
		component(KeepElement),
		toComponent(portrait_div, GameElement)
	)

	return portrait_div
}



export function setPlayerPortraitDead(team: number) {
	if (!portrait_1) {return;}
	if (!portrait_2) {return;}
	if (!portrait_3) {return;}
	if (!portrait_4) {return;}


	switch (team) {
		case 1: portrait_1.dataset.type = "dead"; portrait_1.style.filter = ""; break;
		case 2: portrait_2.dataset.type = "dead"; portrait_2.style.filter = ""; break;
		case 3: portrait_3.dataset.type = "dead"; portrait_3.style.filter = ""; break;
		case 4: portrait_4.dataset.type = "dead"; portrait_4.style.filter = ""; break;
	}
}

export function setPlayerPortraitDisabled(team: number) {
	if (!portrait_1) {return;}
	if (!portrait_2) {return;}
	if (!portrait_3) {return;}
	if (!portrait_4) {return;}


	switch (team) {
		case 1: portrait_1.dataset.type = "disabled"; portrait_1.style.filter = ""; break;
		case 2: portrait_2.dataset.type = "disabled"; portrait_2.style.filter = ""; break;
		case 3: portrait_3.dataset.type = "disabled"; portrait_3.style.filter = ""; break;
		case 4: portrait_4.dataset.type = "disabled"; portrait_4.style.filter = ""; break;
	}
}

export function setPlayerPortraitColor(team: number, color:	string){
	if (!portrait_1) {return;}
	if (!portrait_2) {return;}
	if (!portrait_3) {return;}
	if (!portrait_4) {return;}


	switch (team) {
		case 1: portrait_1.style.filter = getFilter("overlay-multiply", color); portrait_1.dataset.type = "clothes"; break;
		case 2: portrait_2.style.filter = getFilter("overlay-multiply", color); portrait_2.dataset.type = "clothes"; break;
		case 3: portrait_3.style.filter = getFilter("overlay-multiply", color); portrait_3.dataset.type = "clothes"; break;
		case 4: portrait_4.style.filter = getFilter("overlay-multiply", color); portrait_4.dataset.type = "clothes"; break;
	}
}




function updateGameTimers() {
	let cur_time = performance.now()

	let difference = cur_time - start_performance_time
	let seconds = MAX_TIME_LENGTH - Math.round(difference / 1000)


	if (seconds <= 0){
		clearInterval(timerInterval)

		return
	}

	updateTimer(seconds)
	updateTimerBar(seconds)

}

export function startGameTimer() {
	start_performance_time = performance.now()
	updateGameTimers()
	timerInterval = setInterval(updateGameTimers, 500);
}


export function updateTimer(seconds: number) {
	if (time_number_elem) {
		let minutes = Math.floor(seconds / 60)

		time_number_elem.innerHTML = "" + minutes + ":" + String(Math.floor(seconds % 60)).padStart(2, "0")
	}
}

export function updateTimerBar(seconds: number) {
	if (time_bar_elem) {
		time_bar_elem.value = seconds
	}
}

socket.on("game-starting", () => {
	clearInterval(lobby_interval)
	startGameTimer()
})

socket.on("game-ending", () => {
	clearInterval(timerInterval)
})



function onClickFx(fx_button: HTMLButtonElement) {
	toggleMuteFX()
	fx_button.dataset.fx = String(!FX_MUTED)
}

function onClickMusic(music_button: HTMLButtonElement) {
	toggleMuteMusic()
	music_button.dataset.music = String(!MUSIC_MUTED)
}

function leaveGame(world: World) {
	socket.emit('leave-game', () => {
		resetWorld(() => {
			createMenuScene(world)
		})
	})
}
