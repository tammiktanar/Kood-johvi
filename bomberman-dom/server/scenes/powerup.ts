import {GridPosition, PowerUp, Size} from "../schemas";
import {component, World} from "@javelin/ecs";
import {BLOCK_SIZE} from "../env";
import {cloneComponent} from "../lib";

export function spawnPowerUp(world: World, gridPos: {x: number, y: number}) {
	world.create(
		component(PowerUp, {type: Math.floor(Math.random() * 3)}),
		cloneComponent(GridPosition, gridPos),
		component(Size, {width: BLOCK_SIZE, height: BLOCK_SIZE}),
	)
}
