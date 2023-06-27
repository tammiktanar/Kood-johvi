export default class Sound {
	/** @type {GainNode} */
	static #gain

	/** @type {GainNode} */
	static #musicGain
	/** @type {AudioBufferSourceNode} */
	static #musicSource
	/** @type {AudioContext} */
	static #soundContext = (function() {
		const context = new AudioContext()

		Sound.#gain = context.createGain()
		Sound.#gain.connect(context.destination)

		Sound.#musicGain = context.createGain()
		Sound.#musicGain.connect(Sound.#gain)

		Sound.#musicSource = context.createBufferSource()
		Sound.#musicSource.connect(Sound.#musicGain)

		return context
	})()


	/** Plays music on loop at the given url. If music is already playing, replace the currently playing one.
	 * @param {string} url
	 * @param {number} [volume=1] */
	static playMusic(url, volume = 1) {
		const request = new XMLHttpRequest()

		request.open("GET", url, true)
		request.responseType = "arraybuffer"

		request.onload = function() {
			Sound.#soundContext.decodeAudioData(request.response,
				function(response) {
				Sound.#musicSource.buffer = response
				Sound.#musicGain.gain.value = volume
				Sound.#musicSource.start(0)
				Sound.#musicSource.loop = true
			},
				function () {
					console.error(`Failed to play ${url}`)
				})
		}

		request.send()
	}

	/** Pause the music */
	static pause() {
		Sound.#soundContext.suspend()
	}

	/** Resume the music */
	static resume() {
		Sound.#soundContext.resume()
	}

	/**
	/** @type {{[AudioBufferSourceNode,GainNode]}} */
	static #audioElements = {}

	static preloadSound(url) {
		if (url in Sound.#audioElements) return

		const audio = new Audio(url)
		const source = Sound.#soundContext.createMediaElementSource(audio)
		source.connect(Sound.#gain)
		Sound.#audioElements[url] = audio
	}


	static playSound(url, volume=1) {
		/** @type {HTMLAudioElement} */
		let audio

		Sound.preloadSound(url)

		if (url in Sound.#audioElements) {
			audio = Sound.#audioElements[url]
		} else {
			audio = new Audio(url)

			const source = Sound.#soundContext.createMediaElementSource(audio)
			source.connect(Sound.#gain)
			Sound.#audioElements[url] = audio
		}

		audio.pause()
		audio.currentTime = 0
		audio.volume = volume
		audio.play()
			.catch(() => console.error(`Failed to play ${url}`))
	}

	static setVolume(volume = 1) {
		Sound.#gain.gain.value = volume
	}
}

