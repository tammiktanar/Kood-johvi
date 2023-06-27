// noinspection JSUnusedGlobalSymbols

import {Node} from "./engine.js"

// Manages scenes by keeping track of them in a stack-like fashion.
export default class SceneManager extends Node {
	/** @param {Options} opts - The {@link Options} that get applied to this node. Same ones as a default Node.*/
	constructor(opts = {}) {
		super(opts)
	}

	/** @type {[function(): void]} */
	#promiseResolvers = []

	/** Can't manually add children to a SceneManager. Use {@link addScene} or {@link setScene} instead. */
	addChild(_) {throw "Can't manually add children to a SceneManager. Use addScene or setScene instead."}

	_removeChild(child) {
		const i = super._removeChild(child)
		const [resolver] = this.#promiseResolvers.splice(i, 1)
		resolver()
	}

	_triggerKey(event) {
		this.keyHandler(event)

		const children = this.children
		const len = children.length

		if (len > 0)
			children[len - 1]._triggerKey(event)
	}


	/** Takes a scene function and pushes it onto the scene stack. Also returns a promise that is resolved when the new scene is unloaded.
	 * @param {function(SceneManager, ...*)} sceneFunc
	 * @param {...*} args
	 * @returns {Promise} */
	addScene(sceneFunc, ...args) {
		super.addChild(sceneFunc(this, ...args))

		let [promise, resolver] = createPromise()
		this.#promiseResolvers.push(resolver)
		return promise
	}

	/** Clears all scenes from the stack */
	clearScene() {
		this.children.forEach(child => child.unlink())
	}

	/** Clears the scene stack and then sets a new scene. Also returns a promise that is resolved when the new scene is unloaded.
	 * @param {function(SceneManager, ...*)} sceneFunc
	 * @param {...*} args
	 * @returns {Promise} */
	setScene(sceneFunc, ...args) {
		this.clearScene()
		return this.addScene(sceneFunc, ...args)
	}
}

function createPromise() {
	let resolver
	return [
		new Promise(resolve => resolver = resolve),
		resolver,
	]
}
