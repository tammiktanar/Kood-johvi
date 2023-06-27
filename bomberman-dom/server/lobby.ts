import {Socket} from "socket.io";
import {io} from "./net";
import {removeNamedListener, socketName} from "./lib";
import {LOBBY_START_DELAY} from "./env";

export class LobbyManager {
	id: number
	open = true
	readonly members = new Set<Member>()
	readonly gameStart: StartFunction
	readonly destroyLobby: ()=>void

	constructor(id: number, gameStartFunc: StartFunction, destroyLobby: ()=>void) {
		this.id = id
		this.gameStart = gameStartFunc
		this.destroyLobby = destroyLobby
	}

	join (socket: Socket) {
		if (!this.tryJoin(socket)) {
			console.log(socketName(socket), "failed to join lobby", this.id)
		}
	}

	tryJoin(socket: Socket): boolean {
		if (!this.open) return false

		if ([...this.members].find(mem => mem.socket === socket))
		{
			return false
		}

		const member = new Member(socket)
		this.members.add(member)
		this.initMember(member)

		this.stopCountdown()
		if (this.members.size === 4) {
			this.open = false
			this.startCountdown()
		}

		socket.leave("global-chat")
		socket.emit("join-lobby-chat")
		socket.data["chat-room"] ="lobby-"+this.id

		this.update()

		return true
	}

	private leave(mem: Member) {
		if (!this.members.has(mem)) return

		this.uninitMember(mem)
		this.members.delete(mem)

		this.open = true
		this.stopCountdown()

		if (this.members.size === 0) {
			this.open = false
			this.destroyLobby()
		}

		mem.socket.join("global-chat")
		mem.socket.emit("join-global-chat")
		mem.socket.data["chat-room"] = "global-chat"

		this.update()
	}

	private update() {
		const state = this.getState()
		// console.log(state)

		io.in("lobby-"+this.id).emit("update-lobby-state", state)
	}

	private getState() {
		return {
			id: this.id,
				countdown: this.timeout != null,
			members: [...this.members].map(mem => {
			return {
				username: mem.socket.data["username"],
				color: mem.socket.data["color"],
				ready: mem.ready,
			}
		}),
		}
	}

	private initMember(mem: Member) {
		const socket = mem.socket

		socket.join("lobby-"+this.id)

		socket.on("leave-lobby", (ok: ()=>void) => {
			console.log(socketName(socket), "leave lobby", this.id)
			this.leave(mem)
			ok()
		})

		socket.on("ready", () => {
			console.log(socketName(socket), "ready in", this.id)
			mem.ready = true
			if (this.members.size >= 2 && this.isAllReady())
				this.startCountdown()
			this.update()
		})

		socket.on("unready", () => {
			console.log(socketName(socket), "unready in", this.id)
			mem.ready = false
			this.stopCountdown()
			this.update()
		})

		socket.on("get-lobby-state", (response: (v: any)=>void) => {
			response(this.getState())
		})

		const dcFn = () => {
			console.log(socketName(socket), "disconnecting in lobby", this.id)
			this?.leave(mem)
		}

		socket.on("disconnect", dcFn)

		mem.dcFn = dcFn
	}

	private uninitMember(mem: Member) {
		const socket = mem.socket

		socket.leave("lobby-"+this.id)

		removeNamedListener(socket, "leave-lobby")
		removeNamedListener(socket, "ready")
		removeNamedListener(socket, "unready")
		removeNamedListener(socket, "get-lobby-state")

		socket.off("disconnect", mem.dcFn)
	}

	private isAllReady(): boolean {
		for (const mem of this.members.values()) {
			if (!mem.ready) return false
		}
		return true
	}

	private timeout: NodeJS.Timeout | undefined

	private startCountdown() {
		clearTimeout(this.timeout)

		console.log("starting countdown in", this.id)
		this.timeout = setTimeout(() => {
			const membersList: Socket[] = [...this.members].map(mem => mem.socket)

			this.members.forEach((mem) => {
				this.uninitMember(mem)
			})
			this.open = false

			this.gameStart(membersList)
			this.destroyLobby()
		}, 1000 * LOBBY_START_DELAY)
	}

	private stopCountdown() {
		console.log(this.id, "stopping countdown in", this.id)
		clearTimeout(this.timeout)
		this.timeout = undefined
	}
}

interface StartFunction {
	(members: Socket[]): void
}

class Member {
	socket: Socket
	ready = false
	dcFn = () => {}

	constructor(socket: Socket) {
		this.socket = socket
	}
}

