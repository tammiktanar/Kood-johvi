import { createQuery, useMonitor, World } from "@javelin/ecs";
import { setPlayerPortraitColor, setPlayerPortraitDead, updateLives } from "../scenes";
import { Bomber, Lives, Team } from "../schemas";


const bomberList = createQuery(Bomber, Team, Lives)
const deadList = createQuery(Bomber, Team).not(Lives)



export function updateUI() {
	useMonitor(bomberList, (bomberID,  [, team ]) => {
		setPlayerPortraitColor(team.num, team.color)
	})
	useMonitor(deadList, (bomberID,  [, team ]) => {
		setPlayerPortraitDead(team.num)
		updateLives(team.num, 0)
	})

	bomberList((bomberID,  [, team, lives ]) => {
		updateLives(team.num, lives.current);
	})
}
