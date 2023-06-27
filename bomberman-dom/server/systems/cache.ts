import {createQuery, Query, useMonitor, World} from "@javelin/ecs";
import {Bomb, Crate, Explosion, GridPosition, Wall} from "../schemas";
import {useCaches, Cache} from "../caches";
import {FieldNumber} from "@javelin/core";

const walls = createQuery(GridPosition, Wall)
const crates = createQuery(GridPosition, Crate)
const bombs = createQuery(GridPosition, Bomb)
const explosions = createQuery(GridPosition, Explosion)

export const cacheSystem = (world: World) => {
	const caches = useCaches()
	cache(world, walls, caches.block, 1)
	cache(world, crates, caches.block, 2)
	cache(world, bombs, caches.bomb, 1)

	useMonitor(explosions, undefined,
		(e, [pos]) => {
			const x = pos.x
			const y = pos.y

			caches.explosion.flags[y][x] = 0
			caches.explosion.ents[y][x] = 0
		})

	return world
}

type CacheQuery =  Query<[
	{x: FieldNumber, y: FieldNumber},
	...any[],
]>

function cache(world: World, query: CacheQuery, cache: Cache, value: number) {
	useMonitor(query,
		(e, [pos]) => {
			const x = pos.x
			const y = pos.y

			cache.flags[y][x] = value
			cache.ents[y][x] = e
		},
		(e, [pos]) => {
			const x = pos.x
			const y = pos.y

			cache.flags[y][x] = 0
			cache.ents[y][x] = 0
		})
}
