const filterCache = new Set<string>()
const cacheElement = document.createElementNS('http://www.w3.org/2000/svg', 'svg')
cacheElement.style.width = "0"
cacheElement.style.height = "0"
document.body.append(cacheElement)

export function getFilter(id: FilterID, color: string) {
	if (filterCache.has(color)) {
		return makeURL(id, color)
	}

	makeFilter(color)
	filterCache.add(color)
	return makeURL(id, color)
}


function makeURL(id: string, color: string) {
	return `url('#${id}-${c(color)}')`
}

function makeFilter(color: string) {
	const svgContent = `
<filter id="overlay-${c(color)}">
	<feFlood flood-color="${color}" result="flood"/>
	<feBlend mode="overlay" in2="SourceGraphic" result="blend1"/>
	<feComposite in="blend1" in2="SourceAlpha" operator="in" result="composite2"/>
</filter>

	<filter id="overlay-multiply-${c(color)}">
		<feFlood flood-color="${color}" result="flood"/>
		<feBlend mode="overlay" in2="SourceGraphic" result="blend1"/>
		<feBlend mode="multiply" in="flood" in2="blend1" result="blend2"/>
		<feComposite in="blend2" in2="SourceAlpha" operator="in" result="composite2"/>
	</filter>
`
	cacheElement.innerHTML += svgContent
}

function c(color: string) {
	return color.replace("#", "")
}

type FilterID = "overlay" | "overlay-multiply"
