// Just some misc functions here

// Random int from min and up to max (max not included)
export function randIntBetween(min, max) {
	return Math.floor(randBetween(min, max))
}

// Random float from min and up to max
export function randBetween(min, max) {
	return lerp(min, max, Math.random())
}

// Linear interpolation between start and end
export function lerp(start, end, t) {
	return start + (end - start) * t
}



// UPDATERS
export function applyGravity(gravity) {
	return function(delta) {
		this.velY += gravity * delta
	}
}

// POST-COLLISION
export function constSpeed(speed) {
	return function() {
		this.vel = this.vel.normalize().scale(speed)
	}
}
