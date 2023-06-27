import {
	clearObservedChanges,
	createImmutableRef,
	createQuery,
	Query, useInterval, useMonitor,
	World
} from "@javelin/ecs";
import {io, TickData} from "../net";
import {
	Bomb,
	Bomber, BombPrimed,
	Collider, Connection,
	Crate, Explosion, Ghost, GrayWall,
	GridPosition, Hidden, IFrame, Input, Lives, Name,
	Position, PowerUp,
	Size, Team, Velocity,
	Wall
} from "../schemas";
import {createMessageProducer, encode} from "@javelin/net";
import {MESSAGE_MAX_BYTE_LENGTH} from "../env";
import {Socket} from "socket.io";

const useProducer = createImmutableRef(() =>
	createMessageProducer({ maxByteLength: MESSAGE_MAX_BYTE_LENGTH }),
)

const updates: Query[] = [
	Wall,
	GrayWall,
	Size,
	Crate,
	GridPosition,
	Collider,
	Name,
	Team,
	Bomb,
	BombPrimed,
	Explosion,
	IFrame,
	Ghost,
	PowerUp,
	Hidden,
].map((schema) => createQuery(schema))

const patches: Query[] = [
	Position,
	Velocity,
	Lives,
	Bomber,
	Input,
].map((schema) => createQuery(schema))

export function netcodeSystem(world: World<TickData>) {
	// Sending data
	const producer = useProducer()

	for (let i = 0; i < updates.length; i++) {
		useMonitor(updates[i], producer.attach, producer.detach)
	}

	for (let i = 0; i < patches.length; i++) {
		useMonitor(patches[i], producer.attach, producer.detach)

		patches[i]((e, comps) => {
			comps.forEach(comp => {
				producer.patch(e, comp)
				clearObservedChanges(comp)
			})
		})
	}

	const packet = producer.take()
	if (packet) {
		io.in("game-"+world.id).emit("ecs-data", encode(packet))
	}
}

export function getInitialPacket(world: World) {
	const producer = createMessageProducer()

	for (let i = 0; i < updates.length; i++) {
		updates[i].bind(world)((e, comps) => {
			producer.attach(e, comps)
		})
	}

	for (let i = 0; i < patches.length; i++) {
		patches[i].bind(world)(producer.attach)
	}

	return producer.take()
}
