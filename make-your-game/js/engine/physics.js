import Vector from "./vector.js"

// Represents a single collision event
export class Collision {
	impulse = 0
	depth = 0
	normal = Vector.ZERO
	/** @type {PhysicsBody} */
	other
}

/**
 * @param {Vector} minA
 * @param {Vector} maxA
 * @param {Vector} minB
 * @param {Vector} maxB
 */
export function aabb(minA, maxA, minB, maxB) {
	return minA.x < maxB.x
		&& maxA.x > minB.x
		&& minA.y < maxB.y
		&& maxA.y > minB.y
}

/**@param {RectCollider} rectA
 * @param {RectCollider} rectB*/
export function rectToRect(rectA, rectB) {
	const minA = rectA.worldPos
	const maxA = rectA.worldPos.add(rectA.sizeVector)

	const minB = rectB.worldPos
	const maxB = rectB.worldPos.add(rectB.sizeVector)

	if (!aabb(minA, maxA, minB, maxB)) return false

	// COLLISION CONFIRMED
	// Now to find closest sides

	const changeLeft = Math.abs(minB.x - maxA.x)
	const changeRight = Math.abs(maxB.x - minA.x)
	const changeUp = Math.abs(minB.y - maxA.y)
	const changeDown = Math.abs(maxB.y - minA.y)

	const arr = [changeLeft, changeRight, changeUp, changeDown]

	let minIndex = 0
	for (let i = 1; i < arr.length; i++) {
		if (arr[i] < arr[minIndex])
			minIndex = i
	}

	const depth = arr[minIndex]
	let normal
	switch (minIndex) {
		case 0: normal = Vector.LEFT; break
		case 1: normal = Vector.RIGHT; break
		case 2: normal = Vector.UP; break
		case 3: normal = Vector.DOWN; break
	}

	resolve(depth, normal, rectA, rectB)
	return true
}

/**@param {RectCollider} rect
 * @param {PlaneCollider} plane*/
export function rectToPlane(rect, plane) {
	const p1 = rect.worldPos
	const p2 = p1.add(rect.sizeVector)

	const p3 = new Vector(p1.x, p2.y)
	const p4 = new Vector(p2.x, p1.y)

	const points = [p1, p2, p3, p4]

	const maxDepth = points.reduce((acc, point) => {
		const x = pointToPlaneDepth(point, plane)
		return x > acc ? x : acc
	}, -Infinity)
	if (maxDepth < 0) return false

	resolve(maxDepth, plane.normal, rect, plane)
	return true
}

/**@param {CircleCollider} circle
 * @param {PlaneCollider} plane*/
export function circleToPlane(circle, plane) {
	const closestPoint = circle.center.sub(plane.normal.scale(circle.radius))

	const depth = pointToPlaneDepth(closestPoint, plane)

	if (depth < 0) return false

	resolve(depth, plane.normal, circle, plane)
	return true
}

/**@param {CircleCollider} circle
 * @param {RectCollider} rect*/
export function circleToRect(circle, rect) {
	const cornerRadius = Math.min(rect.cornerRadius, Math.min(rect.height/2, rect.width/2))

	// eff === effective
	// Circle stuff
	const center = circle.wCenter
	const effRadius = circle.radius + cornerRadius

	// Rect stuff
	const effPosMin = rect.worldPos.addNum(cornerRadius)
	const effPosMax = rect.worldPos.add(rect.sizeVector).addNum(-cornerRadius)

	let closest = new Vector(
		clamp(center.x, effPosMin.x, effPosMax.x),
		clamp(center.y, effPosMin.y, effPosMax.y),
	)

	const toCenter = center.sub(closest)
	const dist2 = toCenter.dot(toCenter)

	if (effRadius ** 2 <= dist2) return

	// COLLISION CONFIRMED

	let normal
	let depth
	if (dist2 === 0) {
		const edgeNormal = pointOnRectEdgeNormal(center, effPosMin, effPosMax)
		if (edgeNormal) {
			// Center is ON edge
			normal = edgeNormal
			depth = effRadius
		} else {
			// Center is not on edge
			const xChange = (center.x - effPosMin.x) < (effPosMax.x - center.x) ? effPosMin.x - center.x : effPosMax.x - center.x
			const yChange = (center.y - effPosMin.y) < (effPosMax.y - center.y) ? effPosMin.y - center.y : effPosMax.y - center.y

			if (Math.abs(xChange) < Math.abs(yChange)) {
				closest.x += xChange
			} else {
				closest.y += yChange
			}

			const toEdge = closest.sub(center)
			const distance = toEdge.length()
			depth = effRadius + distance
			normal = toEdge.divide(distance)
		}
	} else {
		// If circle center is outside the rect
		const distance = Math.sqrt(dist2)
		normal = toCenter.divide(distance)

		depth = effRadius - distance
	}

	resolve(depth, normal, circle, rect)
	return true
}

/**@param {CircleCollider} circleA
 * @param {CircleCollider} circleB*/
export function circleToCircle(circleA, circleB) {
	// The distance at which the circles will collide
	const comboRadius = circleA.radius + circleB.radius
	const comboRadiusSquared = comboRadius ** 2

	const difference = circleA.wCenter.sub(circleB.wCenter)
	const distSquared = difference.dot(difference)
	if (distSquared >= comboRadiusSquared) return
	// COLLISION CONFIRMED

	const distance = Math.sqrt(distSquared)
	let normal
	if (distance === 0) {
		normal = Vector.UP
	} else {
		normal = difference.divide(distance)
	}
	const depth = comboRadius - distance


	resolve(depth, normal, circleA, circleB)
	return true
}

function clamp(num, min, max) {
	return Math.max(min, Math.min(num, max))
}

/**@param {Vector} point
 * @param {Vector} min
 * @param {Vector} max*/
function pointOnRectEdgeNormal(point, min, max) {
	if (point.x === min.x) return Vector.LEFT; else
	if (point.x === max.x) return Vector.RIGHT; else
	if (point.y === min.y) return Vector.UP; else
	if (point.y === max.y) return Vector.DOWN; else
		return undefined
}

/**
 * Resolves a collision, given a depth and normal
 * @param {number} depth
 * @param {Vector} normal
 * @param {PhysicsBody} bodyA
 * @param {PhysicsBody} bodyB
 */
function resolve(depth, normal, bodyA, bodyB) {
	// Calculate relative velocity in terms of the normal direction
	const relativeVel = bodyA.vel.sub(bodyB.vel)
	const impactSpeed = relativeVel.dot(normal)

	const elasticity = Math.min(bodyA.elasticity, bodyB.elasticity)

	// Calculate impulse scalar
	let impulse = 0
	if (relativeVel.dot(normal) < 0)
		impulse = -(1 + elasticity) * impactSpeed / (bodyA.iMass + bodyB.iMass)

	const collision = new Collision()
	collision.impulse = impulse
	collision.depth = depth
	collision.normal = normal
	collision.other = bodyB
	bodyA.resolveCollision(collision)

	collision.normal = Vector.ZERO.sub(collision.normal)
	collision.other = bodyA
	bodyB.resolveCollision(collision)
}

/**
 *
 * @param {Vector} point
 * @param {PlaneCollider} plane
 * @returns {number}
 */
function pointToPlaneDepth(point, plane) {
	const pointToPlane = plane.pos.sub(point)
	return pointToPlane.dot(plane.normal)
}
