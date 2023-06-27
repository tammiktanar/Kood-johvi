import * as Engine from "../engine/engine.js"
import Vector from "../engine/vector.js"

export default function boundingPlanes(width, height) {
	const root = new Engine.Point({
		name: 'bounds-origin',
	})

	root.addChild(new Engine.PlaneCollider({
		name: 'top-bound',
		normal: Vector.DOWN,
	}))

	root.addChild(new Engine.PlaneCollider({
		name: 'left-bound',
		normal: Vector.RIGHT,
	}))

	root.addChild(new Engine.PlaneCollider({
		name: 'right-bound',
		x: width,
		normal: Vector.LEFT,
	}))

	root.addChild(new Engine.PlaneCollider({
		name: 'bottom-bound',
		y: height + 20,
		normal: Vector.UP,
		trigger: true,
		afterCollision: function(collision) {
			collision.other.unlink()
		},
	}))

	return root
}
