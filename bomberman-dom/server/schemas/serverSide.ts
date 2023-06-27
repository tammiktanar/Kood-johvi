import {boolean, number, registerSchema, string} from "@javelin/ecs";

export const Connection = {}

export const Lobby = {
	id: number
}

export const Owner = {
	eid: number
}

registerSchema(Connection, 41, 0)
registerSchema(Lobby, 42)
registerSchema(Owner, 43)
