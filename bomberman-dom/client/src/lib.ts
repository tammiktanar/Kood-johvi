// @ts-ignore
import {playFX} from "./audio";
import {ROOT} from "./globals";

const images = import.meta.glob('../public/imgs/*')

export async function preload() {
	const arr = []
	for (const image of Object.keys(images)) {
		const img = new Image();
		// Slice removes "../public" prefix
		img.src = image.slice(9);

		arr.push(img)
	}

	// @ts-ignore
	window.imageCache = arr
}

export function createDingButton(): HTMLButtonElement {
	const button = document.createElement("button")
	button.addEventListener("click", () => {
		ROOT.focus()
		playFX("ding")
	})
	button.tabIndex = -1
	return button
}
