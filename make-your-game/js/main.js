import * as Engine from "./engine/index.js"

import {updateScoreBarFps} from "./prefabs/scoreBar.js"
import initScene from "./scenes/init.js"

// Create new engine instance
// Use "window." so that it can be accessed from the console
window.game = new Engine.Root({
	element: document.getElementById('game-container')
})

const sceneManager = new Engine.SceneManager({name: "scene-manager"})
game.addChild(sceneManager)

sceneManager.setScene(initScene)


// Config variables
const SIM_RATE = 1 / 120 // The internal simulation rate in seconds
const MIN_FPS = 1 / 30 // If actual FPS drops below this, the game will slow down
const FRAME_DIVIDER = 1  // Divides FPS by this integer
const SLOW_SIM = false // Slow simulation as well as FPS?

// Loop variables
let prevTime = window.performance.now()
let simTime = 0
let accumulator = 0
let frameCount = 0

// // FPS Counter - Bit of a hacky hardcode
// const fpsCounter = new Engine.DomNode({
// 	name: "fps",
// 	styleString: "z-index: 1; margin-left: 2px;"
// })
// game.addChild(fpsCounter)

window.requestAnimationFrame(gameLoop)
function gameLoop(time) {
	if (time === prevTime) {
		window.requestAnimationFrame(gameLoop)
		return
	}

	if (FRAME_DIVIDER > 1 && frameCount++ % FRAME_DIVIDER !== 0) {
		if (SLOW_SIM) prevTime = time
		window.requestAnimationFrame(gameLoop)
		return
	}
	frameCount = 1

	if (document.hidden) {
		prevTime = time
		window.requestAnimationFrame(gameLoop)
		return
	}

	const frameTime = Math.min(MIN_FPS, (time - prevTime) / 1000)
	accumulator += frameTime

	while (accumulator >= SIM_RATE) {
		game.update(SIM_RATE, simTime)
		accumulator -= SIM_RATE
		simTime += SIM_RATE
	}

	const fps = (1000 / (time - prevTime)).toFixed(1)
	// const simSpeed = (100 * Math.min(1, MIN_FPS / ((time - prevTime) / 1000))).toFixed(0)
	// fpsCounter.element.innerText = `FPS: ${fps}\nSim speed: ${simSpeed}%`

	updateScoreBarFps(fps)

	prevTime = time
	window.requestAnimationFrame(gameLoop)
}
