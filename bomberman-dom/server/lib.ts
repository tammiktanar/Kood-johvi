import {Socket} from "socket.io";
import {FieldExtract, Schema} from "@javelin/core";
import {component, ComponentOf} from "@javelin/ecs";

export function randomIntFromInterval(min: number, max: number) { // min and max included
	return Math.floor(Math.random() * (max - min + 1) + min)
}

export function removeNamedListener(socket: Socket, name: string) {
	socket.listeners(name).forEach((listener) => {
		socket.off(name, listener)
	})
}

export function approach(goal: number, current: number, delta: number) {
	const difference = goal - current

	if (difference > delta) {
		return current + delta
	}

	if (difference < -delta) {
		return current - delta
	}

	return goal
}

export function socketName(socket: Socket): string {
	return socket.data["username"] || socket.id
}

export function cloneComponent<$Schema extends Schema>(schema: $Schema, props: Partial<FieldExtract<$Schema>>): ComponentOf<$Schema> {
	const keys = Object.getOwnPropertyNames(props)
	const entries = keys.map(k => [k, props[k]])
	const clone = Object.fromEntries(entries)

	return component(schema, clone)
}
