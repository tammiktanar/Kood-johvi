import * as Engine from "../engine/engine.js"

let time = 0

const scoreBoard = new Engine.DomNode({
	name: "scoreBoard",
})

const panelScoreBoardPanel = new Engine.DomNode({
	name: "panelDiv"
})

const scoreElem = new Engine.DomNode({
	name: "scoreDiv"
})

const livesElem = new Engine.DomNode({
	name: "livesDiv"
})

const fpsElem = new Engine.DomNode({
	name: "fpsDiv"
})

const levelElem = new Engine.DomNode({
	name: "levelDiv"
})

const timerElem = new Engine.DomNode({
	name: "timerDiv"
})



panelScoreBoardPanel.element.innerText = "\n"

scoreBoard.addChild(panelScoreBoardPanel)
scoreBoard.addChild(livesElem)
scoreBoard.addChild(fpsElem)
scoreBoard.addChild(scoreElem)
scoreBoard.addChild(levelElem)
scoreBoard.addChild(timerElem)


export function createScoreBar(gameScene){
    gameScene.addChild(scoreBoard)
	updateScoreBarStats(gameScene.gameStats)
}

export function updateScoreBarFps(fps){
	fpsElem.element.innerText = "FPS: " + fps
}

export function updateScoreBarTimer(givenTime = time){
	time = givenTime
	timerElem.element.innerText = "TIME: " + (String(givenTime).padStart(3, '0'))
	time++
}

export function updateScoreBarStats(gameStats){
	let lives = getLives(gameStats)

	scoreElem.element.innerText = "SCORE: " + (String(gameStats.score).padStart(5, '0'))
	levelElem.element.innerText = "LEVEL: " + (String(gameStats.level).padStart(2, '0'))
	livesElem.element.innerText = lives
}

export function getLives(gameStats){
	let lives = ""

	for (let i = 1; i <= gameStats.maxLives; i++) {
		if (i <= gameStats.lives){
			lives += "❤️"
		} else {
			lives += "🖤"
		}
		
	} 

	return lives
}