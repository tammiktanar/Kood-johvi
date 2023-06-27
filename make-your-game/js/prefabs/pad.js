import * as Engine from "../engine/engine.js"
import Vector from "../engine/vector.js";

export function newPad() {

var left = false;
var right = false;

    const padOffset = new Engine.Point({
        name: "padOffset",
        x: 400,
        y: 550,
    })

    //paddle
    const pad = new Engine.RectCollider({
        velocity:2,
        static: true,
        name: 'pad',
        width:120,
        height:20,
        styleString: "background-color: blue;",
        cornerRadius: 90,
    })

    //align the center of paddle with padOffset
    pad.x = -pad.width/2

    //set ball's direction and speed after collision with paddle
    pad.afterCollision = (collision) => {
        let other = collision.other;
        const normalSpeed = other.normalSpeed | 200;
        const fastSpeed = other.fastSpeed | 300;
        const radians = 60*Math.PI/180

        if (other.name != "ball") return;
        let distance = other.center.x - padOffset.x
        distance /= pad.width/2
        let speed = Vector.UP
        speed = speed.rotate(distance*radians)

        if(Math.abs(distance) > 0.8 ) {
            speed = speed.scale(fastSpeed)
        } else {
            speed = speed.scale(normalSpeed)
        }

        if (collision.normal.dot(Vector.DOWN) < 0) {
            speed.y = -speed.y
        }
        collision.other.vel = speed
    }

    padOffset.keyHandler = (event) => {
        switch (event.code) {
            case "ArrowLeft" :
            case "KeyA":
                left = event.type === "keydown"
                break;
            case "ArrowRight":
            case "KeyD":
                right = event.type === "keydown"
                break;
        }
    }

    padOffset.addUpdater((delta) => {
        if(left && padOffset.x > 0+pad.width/2) {
            padOffset.x -= 300*delta
        }
        if(right && padOffset.x < 800-pad.width/2) {
            padOffset.x += 300*delta
        }
    })

    padOffset.addChild(pad);
    return padOffset;
}