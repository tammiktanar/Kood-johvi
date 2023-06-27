export const PRODUCTION = Boolean(process.env.PRODUCTION)

export const TICK_RATE = Number(process.env.TICK_RATE) || 60
export const PORT = Number(process.env.PORT) || (PRODUCTION ? 8080 : 8000)
export const ENTITY_COUNT = Number(process.env.ENTITY_COUNT) || 1000
export const MESSAGE_MAX_BYTE_LENGTH = Number(process.env.MESSAGE_MAX_BYTE_LENGTH) || Infinity

export const CORS_ORIGIN = process.env.CORS_ORIGIN || ""

export const PLAYER_ACCELERATION = 150
export const PLAYER_MOVEMENT_SLOPE = 200
export const POWERUP_CHANCE = 30

export const LOBBY_START_DELAY = 10
export const GAME_DURATION = 180
export const CLOSING_START = 140
export const CLOSING_DELAY = 0.25
export const CLOSING_LAYERS = 2

export const GAME_END_DELAY = 1

export const BLOCK_SIZE = 30

export const MAP_WIDTH = 15
export const MAP_HEIGHT = 13
