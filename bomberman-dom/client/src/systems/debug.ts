import {createQuery, useInterval, World} from "@javelin/ecs";
import {Bomber, Team} from "../schemas";

// const bombers = createQuery(Bomber, Team)

export function debugSystem(_world: World) {
	// if (!useInterval(1000)) return

	// bombers((eid, [, team]) => {
	// 	console.log(eid, team)
	// 	if (team.num !== 1) return
	//
	// 	console.log(world.storage.getAllComponents(eid))
	// })
}
