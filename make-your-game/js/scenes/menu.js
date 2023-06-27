import * as Engine from "../engine/index.js"
import pauseScene from "./pause.js"
import gameScene from "./game.js"

export default function menuScene(sceneManager) {
	const scene = new Engine.DomNode({
		name: "menu-scene"
	})

	const sceneElement = scene.element
	sceneElement.classList.add("fill")

	// language=HTML
	sceneElement.innerHTML = "" +
		"<div class='menu-title'>BRICK BREAKER</div>" +
		"<div class='menu-buttons button-container'>" +
			"<div class='menu-start button'>START</div>" +
			"<div class='menu-high-scores button'>HIGH SCORES</div>" +
		"</div>"

	const startButton = sceneElement.querySelector(".menu-start")
	startButton.onclick = () => sceneManager.setScene(gameScene)

	return scene
}

