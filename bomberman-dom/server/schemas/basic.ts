// noinspection ES6UnusedImports,DuplicatedCode

import {number, registerSchema} from "@javelin/ecs"
import type {} from "@javelin/core" // Prevents a bug

export const Position = {
    x: number,
    y: number,
}

export const Velocity = {
	x: number,
	y: number,
}

export const VelocityGoal = {
	x: number,
	y: number,
	speed: number,
}

export const Size = {
    width: number,
    height: number,
}

export const Timer = {
	start: number,
	end: number,
}

export const TimerFinished = {}

export const GridPosition = {
	x: number,
	y: number,
}

export const Hidden = {}

registerSchema(Position, 1)
registerSchema(Velocity, 2)
registerSchema(Size, 3)
registerSchema(Timer, 4)
registerSchema(GridPosition, 5)
registerSchema(TimerFinished, 6)
registerSchema(VelocityGoal, 7)
registerSchema(Hidden, 8)
