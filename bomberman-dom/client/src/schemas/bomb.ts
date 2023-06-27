import {number, registerSchema} from "@javelin/ecs";

export const Bomb = {power: number}
export const BombPrimed = {}

export const Explosion = {
	type: number,
	direction: number,
}

registerSchema(Bomb, 11)
registerSchema(BombPrimed, 12)
registerSchema(Explosion, 13)
