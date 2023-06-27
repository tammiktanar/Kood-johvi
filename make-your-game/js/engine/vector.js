export default class Vector {
	constructor(x = 0, y = 0) {
		this.x = x
		this.y = y
	}

	copy() {
		return new Vector(this.x, this.y)
	}

	length() {
		return Math.sqrt(this.x ** 2 + this.y ** 2)
	}

	normalize() {
		const len = this.length()
		return new Vector(this.x / len, this.y / len)
	}

	/** @param {Vector} vec */
	add(vec) {
			return new Vector(this.x + vec.x, this.y + vec.y)
	}

	/** @param {number} num */
	addNum(num) {
		return new Vector(this.x + num, this.y + num)
	}

	/** @param {Vector} vec */
	sub(vec) {
		return new Vector(this.x - vec.x, this.y - vec.y)
	}

	/** @param {number} scalar */
	scale(scalar) {
		return new Vector(this.x * scalar, this.y * scalar)
	}

	/** @param {number} scalar */
	divide(scalar) {
		return new Vector(this.x / scalar, this.y / scalar)
	}

	/** @param {Vector} vec */
	dot(vec) {
		return this.x * vec.x + this.y * vec.y
	}

	/** @param {Vector} onto */
	project(onto) {
		return onto.scale(this.dot(onto) / onto.dot(onto))
	}

	reflect(normal) {
		return this.sub(this.project(normal).scale(2))
	}

	// In radians
	rotation() {
		return Math.atan2(this.y, this.x)
	}

	// In radians
	rotate(rad) {
		const ret = new Vector()
		ret.x = this.x * Math.cos(rad) - this.y * Math.sin(rad)
		ret.y = this.x * Math.sin(rad) + this.y * Math.cos(rad)
		return ret
	}

	lerp(target, t) {
		return new Vector(lerp(this.x, target.x, t), lerp(this.y, target.y, t))
	}

	static get ZERO() {return new Vector(0, 0)}
	static get UP() {return new Vector(0, -1)}
	static get DOWN() {return new Vector(0, 1)}
	static get RIGHT() {return new Vector(1, 0)}
	static get LEFT() {return new Vector(-1, 0)}
}

function lerp(start, end, t) {
	return start + (end - start) * t
}
