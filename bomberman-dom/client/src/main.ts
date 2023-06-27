import { createStartScreen } from "./scenes";
import "./styles/index.css"

import {world} from "./world";
import {preload} from "./lib";

preload()

let then = 0

window.requestAnimationFrame(gameLoop)
function gameLoop(now: DOMHighResTimeStamp) {
	world.step(now - (then || now))

	then = now

	window.requestAnimationFrame(gameLoop)
}

createStartScreen(world)
