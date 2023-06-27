// noinspection JSUnusedGlobalSymbols

import Vector from "./vector.js"
import * as Physics from "./physics.js"

/**
 * Set defaults of opts in place (i.e. without making a copy)
 * @param {Options} options
 * @param {Object} defaults
 * @returns {Options}
 */
function applyDefaults(options, defaults) {
	const keys = Object.keys(defaults)
	keys.forEach(key => {if (!(key in options)) options[key] = defaults[key]})
	return options
}

/**
 * Options for the various node classes
 * @typedef {Object} Options
 * @description
 * <b>Node</b> options
 *   * `name` - This node's name.
 *   * `updater` - The initial updater that gets called every frame and has access to the node via `this`. Callback params are delta, timestamp and updaterID
 *   * `initializer` - The function that's run when this node is linked to the root
 *   * `cleaner` - The function that's run when this node is unlinked from the root. Also gets the node that got unlinked as an argument.
 *
 * <b>DomNode</b> options
 *   * `element` - The associated DOM element.
 *   * `className` - Class name for element. If not set, use name.
 *   * `styleString` - A CSS string that can be used to set initial styles.
 *
 * <b>Point</b> options
 *   * `position` - Set the position as a vector
 *   * `x` - X coordinate in pixels.
 *   * `y` - Y coordinate in pixels.
 *
 * <b>Rect</b> options
 *   * `width` - Width in pixels.
 *   * `height` - Height in pixels.
 *   * `color` - Sets the background color of this box
 *
 * <b>PhysicsBody</b> options
 *   * `beforeCollision` - Function called before collision is applied. If it returns true, the physics will be skipped. 1st argument is a {@link Collision}
 *   * `afterCollision` - Function called after collision is applied. 1st argument is a {@link Collision}
 *   * `static` - Whether this body moves or is static.
 *   * `trigger` - If true, this won't affect other bodies in collisions.
 *   * `elasticity` - Affects the "bounciness" of this body.
 *   * `mass` - The body's mass, affects collisions
 *   * `velocity` - Set the velocity as a vector
 *   * `velX` - X velocity
 *   * `velY` - Y velocity
 *
 * <b>RectCollider</b> options
 *   * `cornerRadius` - The rectangle's corner radius
 *
 * <b>CircleCollider</b> options
 *   * `radius` - The circle's radius
 *
 * <b>PlaneCollider</b> options
 *   * `normal` - The normal vector for this plane
 *
 * @property {string} [name] - The name
 * @property {function(number, number)} [updater] - Updater function
 * @property {function()} [initializer] - Initializer function
 * @property {function(Node)} [cleaner] - Cleaner function
 * @property {HTMLElement} [element] - HTML element
 * @property {string} [className] - Class name
 * @property {string} [styleString] - Style string
 * @property {Vector} [position] - Position as a vector
 * @property {number} [x] - X coordinate
 * @property {number} [y] - Y coordinate
 * @property {number} [width] - Width
 * @property {number} [height] - Height
 * @property {string} [color] - Background color
 * @property {function(Collision) : boolean} [beforeCollision] - Called before collision.
 * @property {function(Collision)} [afterCollision] - Called after collision.
 * @property {boolean} [static] - Is static?
 * @property {boolean} [trigger] - Is a trigger?
 * @property {number} [mass] - The body's mass
 * @property {number} [elasticity] - The body's elasticity when colliding with other objects
 * @property {Vector} [velocity] - Velocity as a vector
 * @property {number} [velX] - X velocity
 * @property {number} [velY] - Y velocity
 * @property {number} [cornerRadius] - Rect's corner radius
 * @property {number} [radius] - Circle's radius
 * @property {Vector} [normal] - Plane's normal vector
 * */

// Represents a node in the game engine tree.
// Can be used as a helper to group child nodes without modifying their position.
export class Node {
	/** @param {Options} opts - The {@link Options} that get applied to this node. */
	constructor(opts = {}) {
		applyDefaults(opts, {
			name: '',
			updater: undefined,
			initializer: function(){},
			cleaner: function(){},
		})

		this.name = opts.name
		if (opts.updater) this.addUpdater(opts.updater)
		this.initializer = opts.initializer
		this.cleaner = opts.cleaner
	}


	// ----------------
	//  TREE STRUCTURE
	// ----------------
	//region
	/** @type {Node}   */ #parent
	/** @type {[Node]} */ #children = []

	get parent() {return this.#parent}
	/** @returns {[Node]} */
	get children() {return [...this.#children]}

	// The number of children this node has
	get length() {return this.#children.length}

	// Create a string representation of this node and its children
	print(depth = 0) {
		return ['  '.repeat(Math.max(0, depth-1)) + (depth > 0 ? ' └' : '') + this.name,
			...this.#children.map(ch => ch.print(depth+1))]
			.join('\n')
	}

	/**
	 * Set the parent of this node. Returns the parent.
	 * @param {Node} parentNode
	 * @returns {Node}
	 */
	setParent(parentNode) {
		this.unlink()

		// Connect nodes internally
		this.#parent = parentNode
		this.#parent.#children.push(this)

		// Connect any DOM elements
		const parElem = this.#parentElement
		if (parElem === undefined) return parentNode

		this.#childElements.forEach(childElem => parElem.appendChild(childElem))

		if (this.getRoot()) {
			this._triggerInitialize()
			this._triggerUpdate(0, undefined)
		}

		return parentNode
	}

	/**
	 * Add a child to this node. Returns the child.
	 * @param {Node} childNode
	 * @returns {Node}
	 */
	addChild(childNode) {
		childNode.setParent(this)
		return childNode
	}

	// Unlink this node from its parent
	unlink() {
		if (this.#parent === undefined) return

		// Trigger cleanup if being unlinked from root
		if (this.getRoot()) this._triggerCleanup(this)

		this.#parent._removeChild(this)

		// Unlink from DOM
		this.#childElements.forEach(ele => ele.remove())
	}

	/** Remove a child object, and return the index it was at before. Really only for internal engine usage.
	 * @param child
	 */
	_removeChild(child) {
		const i = this.#children.indexOf(child)
		this.#children.splice(i, 1)
		child.#parent = undefined
		return i
	}
	//endregion


	// -------------
	//  DOM HELPERS
	// -------------
	//region

	/**Get first parent in tree with a DOM element
	 * @returns {HTMLElement|undefined}*/
	get #parentElement() {
		if (this.#parent === undefined) {
			return undefined
		} else if ('element' in this.#parent) {
			return this.#parent.element
		} else {
			return this.#parent.#parentElement
		}
	}

	/**Starting from this node, traverse every branch and get the first DOM elements
	 * @returns {HTMLElement[]}*/
	get #childElements() {
		if ('element' in this) return [this['element']]
		return this.#children.flatMap(c => c.#childElements)
	}

	/**A way to get the root node
	 * @returns {Root|undefined}*/
	getRoot() {return this.#parent?.getRoot()}
	//endregion


	// --------
	//  UPDATE
	// --------
	//region

	/* Stop update chain if true */
	pause = false
	timeMult = 1

	/**The functions that are called every update. If they return true, the updater gets removed
	 * @type {[function(number, number)]} */
	#updaters = []

	/** Push an updater to the update stack. Returns ID of updater.
	 * @param {function(number, number)} func - Receives delta time and a timestamp */
	addUpdater(func) {
		this.#updaters.push(func)
	}

	// Remove updater with the given ID.
	#removeUpdater(index) {
		this.#updaters.splice(index, 1)
	}

	// Clears all updaters
	clearUpdaters() {this.#updaters = []}

	// Trigger a cascading update
	_triggerUpdate(delta, time) {
		if (this.pause) return

		for (let i = 0; i < this.#updaters.length; i++) {
			const ret = this.#updaters[i].call(this, delta * this.timeMult, time)
			if (ret === true) {
				this.#removeUpdater(i)
				i--
			}
		}

		for (const child of this.#children)
			child._triggerUpdate(delta * this.timeMult, time)
	}
	//endregion


	// ------------
	//  INITIALIZE
	// ------------
	//region

	/* The function that does the initializing */
	initializer = function(){}

	// Trigger a cascading initialize
	_triggerInitialize() {
		this.initializer()
		for (const child of this.#children)
			child._triggerInitialize()
	}
	//endregion


	// ---------
	//  CLEANUP
	// ---------
	//region

	/** The function that does the cleaning
	 * @type {Node} target - The node that got unlinked and triggered this cleanup */
	cleaner = function(target){}

	// Trigger a cascading cleanup
	_triggerCleanup(target) {
		this.cleaner(target)
		for (const child of this.#children)
			child._triggerCleanup(target)
	}
	//endregion


	// ----------------
	//  KEYBOARD INPUT
	// ----------------
	//region

	/** The function that does the keyboard event handling
	 * @type {function(KeyboardEvent)} */
	keyHandler = function(event){}

	/** Trigger a cascading keyboard event
	 * @param {KeyboardEvent} event
	 */
	_triggerKey(event) {
		this.keyHandler(event)
		for (const child of this.#children)
			child._triggerKey(event)
	}
	//endregion


	// ------
	//  MISC
	// ------
	//region

	/* Node doesn't have a position, so it just gets the parent's world position, if possible */
	get worldPos() {return 'parent' in this ? this.parent.worldPos : NaN}

	_invalidateWorldPos() {
		for (const child of this.#children)
			child._invalidateWorldPos()
	}


	/**
	 * Search from this node downwards for a node with a given name.
	 * @param {string} name
	 * @returns {Node|DomNode|Root|Point|Rect|undefined}
	 */
	findNode(name) {
		if (this.name === name) return this

		const children = this.#children
		for (let i = 0; i < children.length; i++) {
			const res = children[i].findNode(name)
			if (res) return res
		}

		return undefined
	}
	//endregion
}


// DomNode represents a node that has a DOM element attached to it (doesn't necessarily have to be visible)
export class DomNode extends Node {
	/** @param {Options} opts - The {@link Options} that get applied to this node */
	constructor(opts = {}) {
		super(opts)
		applyDefaults(opts, {
			element: document.createElement('div'),
			className: opts.name,
			styleString: '',
		})

		this.element = opts.element

		if (opts.className) this.element.className = opts.className
		this.element.classList.add("absolute")

		if (opts.styleString) this.element.style.cssText = opts.styleString
	}

	get style() {return this.element.style}
}


// The root node of the engine is what all other nodes are connected to
export class Root extends DomNode {
	/** @param {Options} opts - The {@link Options} that get applied to this node */
	constructor(opts = {}) {
		if (!('element' in opts)) throw new Error('need an element for the root node')

		applyDefaults(opts, {
			name: "root",
		})
		super(opts)

		this.element.tabIndex = 0

		this.style.position = 'relative'

		this.element.addEventListener("keydown", event => this._triggerKey(event))
		this.element.addEventListener("keyup", event => this._triggerKey(event))
	}

	/** @type {[PhysicsBody|undefined]} */ dynamicBodies = []
	/** @type {[PhysicsBody|undefined]} */ staticBodies = []

	// Update the whole node tree
	update(delta, time) {
		// TODO: Separate drawing into its own step for some performance gains
		this._triggerUpdate(delta, time)

		if (this.pause) return

		const dynamics = this.dynamicBodies
		const statics = this.staticBodies

		// PHYSICS
		for (const i in dynamics) {
			for (const j in dynamics) {
				if (j <= i) continue
				dynamics[i]?.collideWith(dynamics[j])
			}
			for (const j in statics) {
				dynamics[i]?.collideWith(statics[j])
			}
		}

		// Clean undefined physics bodies
		for (let i = dynamics.length - 1; i >= 0; i--) {
			if (dynamics[i] === undefined) dynamics.splice(i, 1)
		}

		for (let i = statics.length - 1; i >= 0; i--) {
			if (statics[i] === undefined) statics.splice(i, 1)
		}
	}

	getRoot() {return this}

	// World origin is at 0 0
	#worldPos = Vector.ZERO
	get worldPos() {return this.#worldPos}
}


// A point represents a single point in the game's coordinates. Can be used to anchor multiple children conveniently
export class Point extends DomNode {
	/** @param {Options} opts - The {@link Options} that get applied to this node */
	constructor(opts = {}) {
		super(opts)
		applyDefaults(opts, {
			position: undefined,
			x: 0,
			y: 0,
		})

		this.pos = opts.position ? opts.position : new Vector(opts.x, opts.y)
	}

	#pos = Vector.ZERO
	get pos() {return this.#pos}
	set pos(v) {
		this.#pos = v
		this._invalidateWorldPos()
		this.style.transform = `translate(${v.x}px, ${v.y}px)`
	}

	get x() {return this.#pos.x}
	set x(v) {
		this._invalidateWorldPos()
		this.#pos.x = v
		this.style.transform = `translate(${v}px, ${this.y}px)`
	}

	get y() {return this.#pos.y}
	set y(v) {
		this._invalidateWorldPos()
		this.#pos.y = v
		this.style.transform = `translate(${this.x}px, ${v}px)`
	}


	/** @returns {number} */
	get worldX() {return this.worldPos.x}
	/** @returns {number} */
	get worldY() {return this.worldPos.y}

	#worldPos = new Vector()
	#worldPosInvalid = true

	_invalidateWorldPos() {
		this.#worldPosInvalid = true
		super._invalidateWorldPos()
	}

	/** @returns {Vector} */
	get worldPos() {
		if (this.#worldPosInvalid) {
			this.#worldPos = this.parent?.worldPos?.add(this.pos)
			this.#worldPosInvalid = false
		}
		return this.#worldPos
	}
}


// A rect represents a (probably visible) rectangle in the game.
export class Rect extends Point {
	/** @param {Options} opts - The {@link Options} that get applied to this node */
	constructor(opts = {}) {
		super(opts)
		applyDefaults(opts, {
			width: 0,
			height: 0,
		})

		this.width = opts.width
		this.height = opts.height

		this.style.backgroundColor = opts.color
	}

	#size = Vector.ZERO
	get sizeVector() {return this.#size}
	set sizeVector(v) {
		this.#size = v
		this.style.width = v.width + 'px'
		this.style.height = v.height + 'px'
	}

	get width() {return this.#size.x}
	set width(v) {
		this.#size.x = v
		this.style.width = v + 'px'
	}

	get height() {return this.#size.y}
	set height(v) {
		this.#size.y = v
		this.style.height = v + 'px'
	}

	get x2() {return this.x + this.width}
	set x2(v) {this.width = v - this.x}
	get worldX2() {return this.worldX + this.width}

	get y2() {return this.y + this.height}
	set y2(v) {this.height = v - this.y}
	get worldY2() {return this.worldY + this.height}
}

/***********/
/* PHYSICS */
/***********/

// A basic unspecialized physics body
class PhysicsBody extends Rect {
	/** @param {Options} opts - The {@link Options} that get applied to this node */
	constructor(opts = {}) {
		super(opts)
		applyDefaults(opts, {
			static: false,
			trigger: false,
			velX: 0,
			velY: 0,
			velocity: undefined,
			mass: undefined,
			elasticity: 1,

			afterCollision: function(){},
			beforeCollision: function(){},
		})

		/** @type {function(Collision)} */
		this.afterCollision = opts.afterCollision
		/** @type {function(Collision) : boolean} */
		this.beforeCollision = opts.beforeCollision

		this.colliderType = ""

		this.vel = opts.velocity ? opts.velocity : new Vector(opts.velX, opts.velY)

		this.elasticity = opts.elasticity

		this.#static = opts.static
		this.#trigger = opts.trigger

		if (!this.#static) this.element.classList.add("dynamic")

		this.mass = this.isStatic() ? Infinity : opts.mass ? opts.mass : this.width * this.height * (this.width + this.height) / 2 / 10000
	}

	// Prev position for setting static object velocity, in case they are being manually moved
	#prevPos = Vector.ZERO

	#static = false
	isStatic() {return this.#static}

	#trigger = false
	isTrigger() {return this.#trigger}

	get velX() {return this.vel.x}
	set velX(v) {this.vel.x = v}
	get velY() {return this.vel.y}
	set velY(v) {this.vel.y = v}

	#mass = 0
	#iMass = 0
	get mass() {return this.#mass}
	set mass(v) {
		this.#mass = v
		this.#iMass = 1 / v
	}
	get iMass() {return this.#iMass}

	get aabb() {
		return [
			this.worldX,
			this.worldY,
			this.worldX2,
			this.worldY2
		]
	}


	// Trigger a cascading update
	_triggerUpdate(delta, time) {
		super._triggerUpdate(delta, time)
		if (this.pause) return

		if (!this.#static) {
			this.pos = this.pos.add(this.vel.scale(delta))

		} else if (delta > 0) {
			// this.vel = this.worldPos.sub(this.#prevPos).scale(1 / delta)
			// this.#prevPos = this.worldPos
		}
	}

	/** @param {PhysicsBody} body */
	collideWith(body) {}

	/** @param {RectCollider} rect */
	collideWithRect(rect) {}
	/** @param {CircleCollider} circle */
	collideWithCircle(circle) {}
	/** @param {PlaneCollider} plane */
	collideWithPlane(plane) {}

	/** @param {Collision} collision */
	resolveCollision(collision) {
		if (collision.other.isTrigger()) return

		const skipCollision = this.beforeCollision(collision)
		if (skipCollision) return

		if (!this.#static && !this.#trigger) {
			// Separate objects
			this.pos = this.pos.add(collision.normal.scale(collision.depth * this.iMass / (this.iMass + collision.other.iMass)))

			// Adjust velocity
			this.vel = this.vel.add(collision.normal.scale(collision.impulse * this.iMass))
		}

		this.afterCollision(collision)
	}

	_triggerInitialize() {
		super._triggerInitialize()

		if (this.#static) {
			this.#prevPos = this.worldPos
			this.getRoot().staticBodies.push(this)
		} else {
			this.getRoot().dynamicBodies.push(this)
		}
	}

	_triggerCleanup(target) {
		let bodies
		if (this.#static)
			bodies = this.getRoot().staticBodies
		else
			bodies = this.getRoot().dynamicBodies

		bodies[bodies.indexOf(this)] = undefined

		super._triggerCleanup(target)
	}
}

// A rectangle collider. Can have rounded corners, but currently they only work when colliding with circles.
export class RectCollider extends PhysicsBody {
	constructor(opts = {}) {
		super(opts)
		applyDefaults(opts, {
			cornerRadius: 0
		})

		this.colliderType = 'rect'
		/** @type {number} */
		this.cornerRadius = opts.cornerRadius
	}

	#cornerRadius = 0
	get cornerRadius() {return this.#cornerRadius}
	set cornerRadius(v) {
		this.#cornerRadius = v
		this.style.borderRadius = v > 0 ? v + "px" : ""
	}

	collideWith(body) {
		return body?.collideWithRect(this)
	}

	collideWithCircle(circle) {
		return Physics.circleToRect(circle, this)
	}

	collideWithRect(rect) {
		return Physics.rectToRect(this, rect)
	}

	collideWithPlane(plane) {
		return Physics.rectToPlane(this, plane)
	}
}

// A circle collider.
export class CircleCollider extends PhysicsBody {
	/** @param {Options} opts - The {@link Options} that get applied to this node */
	constructor(opts = {}) {
		super(opts)
		applyDefaults(opts, {
			radius: 0
		})

		this.element.classList.add("circle")

		this.radius = opts.radius
		this.mass = this.isStatic() ? Infinity : opts.mass ? opts.mass : Math.PI * this.radius ** 2 / 10000
		this.colliderType = 'circle'
	}

	#radius = 0
	get radius() {return this.#radius}
	set radius(r) {
		const change = r - this.#radius
		this.#radius = r

		this.width = r * 2
		this.height = r * 2

		this.x -= change
		this.y -= change
	}

	/** @returns {Vector} */
	get center() {
		return this.pos.addNum(this.radius)
	}

	get wCenter() {
		return this.worldPos?.addNum(this.radius)
	}

	collideWith(body) {
		return body?.collideWithCircle(this)
	}

	collideWithRect(rect) {
		return Physics.circleToRect(this, rect)
	}

	collideWithPlane(plane) {
		return Physics.circleToPlane(this, plane)
	}

	collideWithCircle(circle) {
		return Physics.circleToCircle(this, circle)
	}
}

// An infinite length plane collider
export class PlaneCollider extends PhysicsBody {
	constructor(opts = {}) {
		opts.static = true
		super(opts)
		applyDefaults(opts, {
			normal: Vector.UP
		})

		this.normal = opts.normal.x === 0 && opts.normal.y === 0 ? Vector.UP : opts.normal.normalize()
		this.colliderType = 'plane'
	}

	collideWith(body) {
		return body?.collideWithPlane(this)
	}

	collideWithRect(rect) {
		return Physics.rectToPlane(rect, this)
	}

	collideWithPlane(plane) {}

	collideWithCircle(circle) {
		return Physics.circleToPlane(circle, this)
	}
}
