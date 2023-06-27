import {registerSchema} from "@javelin/ecs";

export const GameElement = {}
export const KeepElement = {}

export const Grid = {}

registerSchema(KeepElement, 41)

registerSchema(Grid, 42, 1)
registerSchema(GameElement, 43, 0)
