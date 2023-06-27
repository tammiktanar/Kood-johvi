import * as Engine from "../engine/index.js"
import { getLives } from "../prefabs/scoreBar.js"
import gameScene from "./game.js"
import menuScene from "./menu.js"

export default function winScene(sceneManager, gameStats) {
	let lives = getLives(gameStats)


	const scene = new Engine.DomNode({
		name: "win-scene"
	})
    

	const sceneElement = scene.element
	sceneElement.classList.add("fill")

	// language=HTML
	sceneElement.innerHTML = "" +
		"<div class='win-menu fill'>" +
			"<div class='win-title'>YOU WIN</div><br><br>" +
            "<div class='win-score'>YOUR SCORE: "+ gameStats.score +"</div><br>"+
            "<div class='win-lives'>YOU SURVIVED WITH <br>"+ lives +"</div><br>"+
			"<div class='win-buttons button-container'>" +
				"<div class='win-restart button'>RESTART</div>" +
				"<div class='win-exit button'>MAIN MENU</div>" +
			"</div>" +
		"</div>"

	const exitButton = sceneElement.querySelector(".win-exit")
	exitButton.onclick = () => {sceneManager.setScene(menuScene)}

    const restartButton = sceneElement.querySelector(".win-restart")
	restartButton.onclick = () => sceneManager.setScene(gameScene)


	scene.keyHandler = (event) => {
		if (event.type === "keydown" && event.code === "Escape") {
			sceneManager.setScene(menuScene)
		}
	}

	return scene
}

