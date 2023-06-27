// noinspection DuplicatedCode

import {boolean, number, registerSchema, string} from "@javelin/ecs";
import {FieldNumber, FieldString} from "@javelin/core"

export const Bomber = {
	speed: number,
	power: number,
	bombCount: number,
}

export const Input = {
	down: boolean,
	up: boolean,
	left: boolean,
	right: boolean,
	bomb: boolean,
	walk: boolean,
}

export const Team: {num: FieldNumber, color: FieldString} = {
	num: number,

	// @ts-ignore
	color: { ...string, length: 7 },
}

export const Lives = {
	current: number,
	max: number,
}

export const IFrame = {}

// @ts-ignore
export const Name: {s: FieldString} = {s: { ...string, length: 16 }}

export const Ghost = {}

registerSchema(Bomber, 21)
registerSchema(Input, 22)
registerSchema(Team, 23)
registerSchema(Lives, 24)
registerSchema(IFrame, 25)
registerSchema(Name, 26)
registerSchema(Ghost, 27)
