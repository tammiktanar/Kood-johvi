import {number, registerSchema} from "@javelin/ecs";

export const Collider = {}

export const Wall = {}
export const Crate = {}

export const PowerUp = {type: number}

export const GrayWall = {}

export const Closing = {dir: number}

registerSchema(Collider, 31)
registerSchema(Wall, 32)
registerSchema(Crate, 33)
registerSchema(PowerUp, 34)
registerSchema(GrayWall, 35)
registerSchema(Closing, 36)
