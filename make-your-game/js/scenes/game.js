import * as Engine from "../engine/index.js"
import boundingPlanes from "../prefabs/boundingPlanes.js"
import {generateBricks} from "../prefabs/bricks.js"
import {newPad} from "../prefabs/pad.js"
import Vector from "../engine/vector.js"
import pauseScene from "./pause.js"
import {createScoreBar, updateScoreBarStats, updateScoreBarTimer} from "../prefabs/scoreBar.js"
import { gameLevels } from "../prefabs/levels.js"
import winScene from "./win.js"
import gameOver from "./gameOver.js"
import Sound from "../sound/sound.js"

/** @typedef GameStats
 * @property {Number} [maxLives=5]
 * @property {Number} [maxLevel=10]
 * @property {Number} [lives=5]
 * @property {Number} [level=0]
 * @property {Number} [bricks=0]
 * @property {Number} [score=0]
 */

/** @type {Root} root
    @type {GameStats} gameStats */
export default function gameScene(sceneManager, gameStats = {}) {
	applyDefaults(gameStats, {
		maxLives: 5,
		maxLevel: gameLevels.length,
		lives: 5,
		level: 0,
		bricks: 0,
		score: 0,
	})

	const scene = new Engine.DomNode({
		name: "game-scene"
	})

	scene.gameStats = gameStats

	scene.addChild(boundingPlanes(800, 600))

	generateBricks(scene, gameLevels[gameStats.level])
	createScoreBar(scene)
	//paddle
	const paddle = newPad()
	scene.addChild(paddle)

	let balls = 0

	//ball
	const makeBall = () => {
		let launched = false // whether ball has been launched or not, should return to false on death and new level.

		const ball = new Engine.CircleCollider({
			name: "ball",
			radius: 8,
			x: 400,
			y: 500,
			velocity: Vector.UP.scale(250).rotate(0),
			initializer: function () {
				balls++
			},
			cleaner: function (target) {
				// Check for if the ball was removed or the entire game scene got removed
				if (target !== this) return

				balls--
				if (balls <= 0) {
					Sound.playSound("/sound/sfx/dead.wav")
					if (gameStats.lives-- > 0) {
						scene.addChild(makeBall())
					} else {
						sceneManager.addScene(gameOver, gameStats.score)
						scene.pause = true
					}
					updateScoreBarStats(gameStats)
				}
			},
			updater: function() {
				if (!launched) {
					this.x = paddle.x-this.radius;
					this.y = paddle.y -(this.radius*2+1);
				}
			},
			afterCollision: function(collision) {
				const other = collision.other

				 if (other.name === "pad") {
					 Sound.playSound("/sound/sfx/bouncePad.wav")
				 } else if (other.colliderType === "rect") {
					 if (other.maxHp < 0) {
						 // If brick is unbreakable
					    Sound.playSound("/sound/sfx/bounce.wav")
					 } else {
					    Sound.playSound("/sound/sfx/bounceBrick.wav")
					 }
				 } else {
					 Sound.playSound("/sound/sfx/bounce.wav")
				 }
			}
		})

		ball.keyHandler = (event) => { //ball's keyhandler
			if (event.code === "Space" && !launched) {
				launched = true;
				ball.vel = Vector.UP.scale(200).rotate(22*Math.PI/180); //on launch ball goes slightly right, not straight up
			}
		}

		return ball
	}
	scene.addChild(makeBall())

	//scene keyhandler
	scene.keyHandler = function(event) {
		if (event.type === "keydown") {
			switch (event.code) {
				case "Escape":
					scene.pause = true
					Sound.pause()

					const onceUnpaused = sceneManager.addScene(pauseScene)

					onceUnpaused.then(() => {
						scene.pause = false
						Sound.resume()
					})
					break

				case "PageUp":
					gameStats.level++
					sceneManager.setScene(gameScene, gameStats)
					break

				case "PageDown":
					gameStats.level--
					sceneManager.setScene(gameScene, gameStats)
					break

				case "Home":
					this.timeMult = 5
					break
			}
		} else {
			if (event.code === "Home") {
				this.timeMult = 1
			}
		}
	}

	scene.addUpdater(updateGameSceneTimer);

	return scene
}

export function nextLevel(game){
	game.gameStats.level++
	if (game.gameStats.level === game.gameStats.maxLevel) {
		game.parent.setScene(winScene, game.gameStats)
	} else {
		game.parent.setScene(gameScene, game.gameStats)
	}
}

function applyDefaults(options, defaults) {
	const keys = Object.keys(defaults)
	keys.forEach(key => {if (!(key in options)) options[key] = defaults[key]})
	return options
}


function updateGameSceneTimer(delta) {
	if (!this.timer) this.timer = delta
	this.timer += delta

	let time = Math.round(this.timer)
	if(!this.pause) updateScoreBarTimer(time);
}
