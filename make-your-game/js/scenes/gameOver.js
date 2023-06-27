import * as Engine from "../engine/index.js"
import menuScene from "./menu.js"
import gameScene from "./game.js"

export default function gameOver(sceneManager, score) {
	const scene = new Engine.DomNode({
		name: "death-scene"
	})

	const sceneElement = scene.element
	sceneElement.classList.add("fill")

	// language=HTML
	sceneElement.innerHTML = "" +
		"<div class='death-menu'>" +
		"<div class='death-title'>YOU DIED</div>" +
		"<div class='death-score'>YOUR SCORE: "+score+"</div>" +
		"<div class='death-buttons button-container'>" +
		"<div class='death-restart button'>RESTART</div>" +
		"<div class='death-exit button'>EXIT</div>" +
		"</div>" +
		"</div>"

	const restartButton = sceneElement.querySelector(".death-restart")
	restartButton.onclick = () => {sceneManager.setScene(gameScene)}

	const exitButton = sceneElement.querySelector(".death-exit")
	exitButton.onclick = () => {sceneManager.setScene(menuScene)}

	return scene
}
