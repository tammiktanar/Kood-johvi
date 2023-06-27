import * as Engine from "../engine/engine.js"
import { updateScoreBarStats } from "./scoreBar.js";
import { nextLevel } from "../scenes/game.js";



let defaultGenArr = [
    "XXXXXXX",
    "M..X...",
    ".X.X.X.",
    "./.X.X.",
    "./.X.X.",
    "./.X.X.",
    ".|.X.X.",
    ".|.X.X.",
    "r|...X.",
    "R+++++.",
];



/**
 * This function generates set of bricks for the game, with given instructions.
 *
 * @param {ParentNode} gameScene - Where the brick generation starts
 * @param {string} [genArr=default_genArr] - A optional string[], to define what type of bricks to generate how and where
 * @return {Array[Array]} All bricks generated put into an array
 *
 * @example
 *
 *     generateBricks(game)
 *
 *
 *      --- Brick types ---
 *
 *      A - Takes 2 to break
 *      P - Power up brick
 *      D - Debuff brick
 *      M - Moving brick
 *      X - Unbreakable brick
 *      R - Regenerative brick
 *      | - Lowers above brick
 *      / - Lowers above brick, while also adding health
 *      _ - Extends previous brick
 *      + - Extends previous brick, while also adding health
 *
 *      --- Colors ---
 *
 *      b - Blue brick
 *      g - Green brick
 *      r - Red brick
 *      y - Yellow brick
 *      p - Pink brick
 *      w - White brick
 *      h - Grey brick
 *      o - Orange brick
 *
 *      . - Emptiness, void, no bricks
 *
 *
 */

export function generateBricks(gameScene, genArr = defaultGenArr){
    let gameSize = generateGameSize(    genArr.length, genArr[0].split("").length) // Calculate the game size
    let brickXSpace = gameSize.brickXSpace; // The width of space between bricks
    let brickYSpace = gameSize.brickYSpace; // The height of space between bricks
    let brickWidth = gameSize.brickSizeWidth; // The width of a brick
    let brickHeight = gameSize.brickSizeHeight; // The height of a brick
    let wallSpacing = gameSize.wallSpacing; // A small gap from the wall to the bricks
    let bricks = []

    genArr.forEach((row, rowNr) => {
        let brickRow = []
        row.split('').forEach((brick, brickNr, curRow) => {
            let canContinue = true // Wether the brick can be added to the game or not
            let setStatic = (brick == "M") ? false : true; // Set static, if not moving brick
            let speedX = (brick == "M") ? 50 : 0; // Set 0 X velocity speed, if not moving brick

            let curBrick = new Engine.RectCollider({ // Create default brick
            //    name: "brick_" + (1 + rowNr * brickNr).toString(),
                name: "brick",
                static: setStatic,
                x: brickWidth * brickNr + wallSpacing + brickXSpace,
                y: brickHeight * rowNr + wallSpacing + brickYSpace,
                velX: speedX,
                velY: 0,
                width: brickWidth - brickXSpace,
                height: brickHeight - brickYSpace,
            });


            curBrick.maxHp = 1
            curBrick.hp = 1
            curBrick.addUpdater(brickUpdater);
            curBrick.afterCollision = ballCollision;



            switch (brick) {
                case '.': // Empty
                    canContinue = false;
                    break;
                case 'A': // Default brick
                    curBrick.element.classList.add("default-brick")

                    break;
                case 'R': // Regenerative brick
                    curBrick.element.classList.add("regen-brick")
                    curBrick.isRegen = true
                    curBrick.maxHp++
                    curBrick.hp++

                    curBrick.addUpdater(function (delta){ // Add regeneration timer
                        if (!this.timer) {this.timer = delta; return}
                        this.timer = this.timer + delta
                        if (this.timer > (5 + 0.5 * this.maxHp)){
                            this.hp = this.maxHp
                            this.timer = 0
                        }
                    })

                    break;
                case 'P': // Powerup brick
                    curBrick.element.classList.add("power-brick")

                case 'M': // Moving brick
                    curBrick.element.classList.add("moving-brick")

                    break;
                case 'D': // Debuff brick
                    curBrick.element.classList.add("debuff-brick")

                    break;
                case 'X': // Unbreakable brick
                    curBrick.element.classList.add("unbreakable-brick")
                    curBrick.hp = -1
                    curBrick.maxHp= -1
                    break;
                default:
                    switch (brick) {
                        case 'r':   // Red brick
                            curBrick.element.classList.add("red-brick")

                            break;
                        case 'b':   // Blue brick
                            curBrick.element.classList.add("blue-brick")

                            break;
                        case 'g':   // Green brick
                            curBrick.element.classList.add("green-brick")

                            break;
                        case 'y':   // Yellow brick
                            curBrick.element.classList.add("yellow-brick")

                            break;
                        case 'p':   // Pink brick
                            curBrick.element.classList.add("pink-brick")
                            break;
                        case 'w':   // White brick
                            curBrick.element.classList.add("white-brick")
                            break;
                        case 'h':   // Grey brick
                            curBrick.element.classList.add("light-grey-brick")
                            break;
                        case 'o':   // Orange brick
                            curBrick.element.classList.add("orange-brick")
                            break;
                        default:
                            canContinue = false
                            break;
                    }
                    break;
            }


            if (canContinue){
                if (brick != "X") gameScene.gameStats.bricks++

                let checkWidthRes =  checkWidth(curRow, brickNr); // See if the width should be extended and by how much
                let checkBrickWidth = checkWidthRes[0] // Check the normal width
                let checkBrickStrongWidth = checkWidthRes[1] // Check the strong width

                let checkHeightRes = checkHeight(genArr, rowNr, brickNr, checkBrickStrongWidth, checkBrickWidth) // See if the height should be extended and by how much
                let checkBrickHeight = checkHeightRes[0] // Check the normal height
                let checkBrickCurStrongHeight = checkHeightRes[1] // Check the strong height

                curBrick.maxHp = curBrick.maxHp * (checkBrickStrongWidth + 1 + checkBrickCurStrongHeight) // Increase max hp from strong width
                curBrick.hp = curBrick.maxHp
                curBrick.width = (brickWidth * (checkBrickWidth + checkBrickStrongWidth)) - brickXSpace, // Extend the width
                curBrick.height = (brickHeight * (checkBrickHeight)) - brickYSpace // Extend the height

                gameScene.addChild(curBrick);
                brickRow.push(curBrick);
            }

        });


        bricks.push(brickRow)
    });








    return bricks
}




function checkWidth(curRow, brickNr){
    // Extend the bricks width
    let checkWidth = true
    let checkBrickWidth = 1
    let checkWidthIndex = 1
    let checkBrickStrongWidth = 0

    while (checkWidth) {
        if ((curRow.length) > brickNr+checkWidthIndex) {
            if (curRow[brickNr + checkWidthIndex] == "_") {
                checkBrickWidth++
            } else {
                if (curRow[brickNr + checkWidthIndex] == "+") {
                    checkBrickStrongWidth++
                } else {
                    checkWidth = false
                }
            }
        } else{
            checkWidth = false
        }

        checkWidthIndex++
    }

    return [checkBrickWidth, checkBrickStrongWidth]
}

function checkHeight(genArr, rowNr, brickNr, checkBrickStrongWidth, checkBrickWidth){
    // Extend the bricks height
    let checkHeight = true
    let checkBrickHeight = 1
    let checkHeightIndex = 1
    let checkBrickStrongHeight = 0
    let checkHeightWidth = true
    let checkHeightWidthTemp = true
    let checkHeightWidthIndex = 0

    while (checkHeight) {
        checkHeightWidthTemp = checkHeightWidth
        if ((genArr.length) > rowNr+checkHeightIndex){ // If there is a row below it
            let checkBrickCurStrongHeight = 0
            while (checkHeightWidthTemp) {
                if ((checkBrickStrongWidth + checkBrickWidth) > checkHeightWidthIndex) { // If the current width isn't the width of the brick
                    if (genArr[rowNr+checkHeightIndex].split('')[brickNr+checkHeightWidthIndex] == "|"){

                        checkHeightWidthIndex++
                    } else if (genArr[rowNr+checkHeightIndex].split('')[brickNr+checkHeightWidthIndex]  == "/" ){

                        checkBrickCurStrongHeight++
                        checkHeightWidthIndex++

                    } else { // If not, end it

                        checkHeight = false
                        checkHeightWidth = false
                        checkHeightWidthTemp = false
                    }
                }

                if ((checkBrickStrongWidth + checkBrickWidth) == checkHeightWidthIndex) { // If the width of the brick has been checked


                    checkBrickStrongHeight += checkBrickCurStrongHeight
                    checkBrickHeight++
                    checkHeightWidthIndex = 0
                    checkHeightWidthTemp = false
                }
            }
        } else {

            checkHeight = false
        }
        checkHeightIndex++
    }


    return [checkBrickHeight, checkBrickStrongHeight]
}



function generateGameSize(rows, columns){
    //Calculate the brick sizes
    let screenSize = {
        width: 800,
        height: 600,
    }

    let brickSizeWidth = Math.round(screenSize.width / (1.0113780025284451 * columns))
    let brickSizeHeight = Math.round(screenSize.height / (1.4 * rows))
    let wallSpacing = 4
    let brickSpacingWidth = 0
    let brickSpacingHeight = 0

    let res = {
        brickXSpace: brickSpacingWidth,
        brickYSpace: brickSpacingHeight,
        brickSizeWidth: brickSizeWidth,
        brickSizeHeight: brickSizeHeight,
        wallSpacing: wallSpacing,
    }


    return res
}






function brickUpdater(){ // Add health & debug updater
    if (this.hp <= 0 && this.maxHp > 0) {
        let game = this.parent
        let score = this.maxHp * (50 * (game.gameStats.level+1))

        game.gameStats.score += score


        // Create Floating score
        let scoreFloat = new Engine.Point({
            name: "score",
            x: this.x,
            y: this.y
        })

        scoreFloat.addUpdater(scoreRemoverUpdater)
        scoreFloat.element.innerHTML = "+" +  score
        this.parent.addChild(scoreFloat)

        game.gameStats.bricks -= 1
        if (game.gameStats.bricks <= 0) {
            nextLevel(game)
        }
        updateScoreBarStats(game.gameStats)
        this.unlink()
    }
    if (this.maxHp < this.hp) this.hp = this.maxHp;
    if (this.maxHp > 0) {
        let healthPerc = this.hp / this.maxHp
        this.element.style.filter = "brightness("+healthPerc+")"
    }
}


function scoreRemoverUpdater(delta){
    if (!this.timer) {this.timer = delta; return}
    this.timer = this.timer + delta
    if (this.timer > 2){
        this.unlink()
    }
}

function ballCollision(collision){ // Add ball/brick colision detection
    let other = collision.other;
    if (other.name.includes("brick_")) {
        if (this.isRegen){
            this.style.backgroundColor = "red"
        }
    }
    if (other.name != "ball") return;
    if (this.isRegen) this.timer = 0;
    this.hp--

}
