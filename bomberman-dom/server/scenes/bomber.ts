import {component, World} from "@javelin/ecs";
import {
	Bomber,
	Input,
	Lives,
	Position,
	Size,
	Team,
	Velocity,
	VelocityGoal
} from "../schemas";
import {BLOCK_SIZE} from "../env";

export const addBomber = (world: World, num: number, x: number, y: number) => {
	return world.create(
		component(Bomber, {speed: 100, power: 2, bombCount: 1}),
		component(Lives, {current: 3, max: 3}),
		component(Team, {num: num}),
		component(Input),
		component(Position, {
			x: BLOCK_SIZE * x + BLOCK_SIZE * 0.1,
			y: BLOCK_SIZE * y + BLOCK_SIZE * 0.1,
		}),
		component(Size, {
			width: BLOCK_SIZE - 8,
			height: BLOCK_SIZE - 8,
		}),
		component(Velocity),
		component(VelocityGoal),
	)
}
