// noinspection JSUnusedGlobalSymbols

import {createImmutableRef, createRef} from "@javelin/ecs";
import {MAP_HEIGHT, MAP_WIDTH} from "./env";

export interface Caches {
	bomb: Cache<CFlagBomb>,
	block: Cache<CFlagBlock>,
	explosion: Cache<CFlagExplosion>,
}

export function useCaches(): Caches {
	return {
		block: useBlockCache(),
		bomb: useBombCache(),
		explosion: useExplosionCache(),
	}
}

export interface Cache<T extends number = number> {
	name: string,
	flags: T[][],
	ents: Uint32Array[],
}

export const useBlockCache = createCache<CFlagBlock>("Bomb Cache")

export enum CFlagBlock {
	Empty = 0,
	Wall,
	Crate,
}

export const useBombCache = createCache<CFlagBomb>("Bomb Cache")

export enum CFlagBomb {
	Empty = 0,
	Exists = 1,
}

export const useExplosionCache = createCache<CFlagExplosion>("Bomb Cache")

export enum CFlagExplosion {
	Empty = 0,
	Cross,
	Line,
	End,
}


function createCache<flagType extends number>(name: string) {
	return createImmutableRef<Cache<flagType>>(() => {
		return {
			name: name,
			flags: Array(MAP_HEIGHT).fill([]).map(_ => new Array(MAP_WIDTH).fill(0)),
			ents: Array(MAP_HEIGHT).fill([]).map(() => new Uint32Array(MAP_WIDTH)),
		}
	}, {shared: true})
}
