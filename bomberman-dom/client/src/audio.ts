import * as howler from "howler";

type KeyFX = "ding" | "explosion"
type KeyMusic = "menu" | "game"

export let FX_MUTED = localStorage.getItem('audio_fx_muted') === 'true'
export let MUSIC_MUTED = localStorage.getItem('audio_music_muted') === 'true'
export let VOLUME = Number(localStorage.getItem('audio_volume') ?? 1)

const FX_MAP = new Map<KeyFX, howler.Howl>()
const MUSIC_MAP = new Map<KeyMusic, howler.Howl>()

function addFX(key: KeyFX, ...paths: string[]) {
	const howl = new howler.Howl({
		src: paths,
		volume: VOLUME,
		mute: FX_MUTED,
	})
	FX_MAP.set(key, howl)
	return howl
}

function addMusic(key: KeyMusic, ...paths: string[]) {
	const howl = new howler.Howl({
		src: paths,
		volume: VOLUME,
		mute: MUSIC_MUTED,
		loop: true,
	})
	MUSIC_MAP.set(key, howl)
	return howl
}

let initDone = false
export function initAudio() {
	if (initDone) return
	initDone = true

	console.log(localStorage.getItem('audio_music_muted'), MUSIC_MUTED)

	addFX("ding", "/sound/ding.mp3")
	addFX("explosion", "/sound/explosion.mp3")

	addMusic("menu", "/sound/menu-music.mp3")
	addMusic("game", "/sound/game-music.mp3")
}

export function playFX(key: KeyFX) {
	FX_MAP.get(key)?.play()
}

let currentlyPlaying: undefined | string
export function playMusic(key: KeyMusic) {
	if (key === currentlyPlaying) return
	currentlyPlaying = key

	stopMusic()
	MUSIC_MAP.get(key)?.play()
}

export function stopMusic() {
	MUSIC_MAP.forEach(v => v.stop())
}

export function toggleMuteFX(force?: boolean) {
	FX_MUTED = force ?? !FX_MUTED
	localStorage.setItem('audio_fx_muted', String(FX_MUTED))

	FX_MAP.forEach(v => v.mute(FX_MUTED))
}

export function toggleMuteMusic(force?: boolean) {
	MUSIC_MUTED = force ?? !MUSIC_MUTED
	localStorage.setItem('audio_music_muted', String(MUSIC_MUTED))

	MUSIC_MAP.forEach(v => v.mute(MUSIC_MUTED))
}

export function setVolume(vol: number) {
	VOLUME = vol
	localStorage.setItem('audio_volume', String(MUSIC_MUTED))

	FX_MAP.forEach(v => v.volume(vol))
	MUSIC_MAP.forEach(v => v.volume(vol))
}



