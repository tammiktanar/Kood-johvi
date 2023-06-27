import * as Engine from "../engine/index.js"
import menuScene from "./menu.js"
import gameScene from "./game.js"

export default function pauseScene(sceneManager) {
	const scene = new Engine.DomNode({
		name: "pause-scene"
	})

	const sceneElement = scene.element
	sceneElement.classList.add("fill")

	// language=HTML
	sceneElement.innerHTML = "" +
		"<div class='pause-menu'>" +
			"<div class='pause-title'>PAUSED</div>" +
			"<div class='pause-buttons button-container'>" +
				"<div class='pause-resume button'>RESUME</div>" +
				"<div class='pause-restart button'>RESTART</div>" +
				"<div class='pause-exit button'>EXIT</div>" +
			"</div>" +
		"</div>"

	const resumeButton = sceneElement.querySelector(".pause-resume")
	resumeButton.onclick = () => {scene.unlink()}

	const restartButton = sceneElement.querySelector(".pause-restart")
	restartButton.onclick = () => {sceneManager.setScene(gameScene)}

	const exitButton = sceneElement.querySelector(".pause-exit")
	exitButton.onclick = () => {sceneManager.setScene(menuScene)}

	scene.keyHandler = (event) => {
		if (event.type === "keydown" && event.code === "Escape") {
			scene.unlink()
		}
	}

	return scene
}
