import * as Engine from "../engine/index.js"
import menuScene from "./menu.js"
import Sound from "../sound/sound.js"

export default function initScene(sceneManager) {
	const scene = new Engine.DomNode({
		name: "init-scene"
	})

	const sceneElement = scene.element
	sceneElement.classList.add("fill")

	// language=HTML
	sceneElement.innerHTML = "<div class='init-button button'>START</div>"

	const startButton = sceneElement.querySelector(".init-button")
	startButton.onclick = () => {
		Sound.setVolume(0.25)
		Sound.playMusic("/sound/music.mp3", 0.6)

		sceneManager.setScene(menuScene)
	}

	preloadSounds()

	return scene
}

function preloadSounds() {
	Sound.preloadSound("/sound/music.mp3")

	Sound.preloadSound("/sound/sfx/bounce.wav")
	Sound.preloadSound("/sound/sfx/bounceBrick.wav")
	Sound.preloadSound("/sound/sfx/bouncePad.wav")
	Sound.preloadSound("/sound/sfx/dead.wav")
}

