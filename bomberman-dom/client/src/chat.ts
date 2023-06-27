import {CHAT_INPUT, CHAT_MESSAGES, ROOT} from "./globals";
import {socket, stopInputs} from "./net";

export function createChatMessage(data: any) {
	let message_div = document.createElement("div") as HTMLDivElement
	let username = document.createElement("p") as HTMLParagraphElement
	let message = document.createElement("p") as HTMLParagraphElement

	username.textContent = data.username
	message.textContent = data.message

	username.style.color = data.color

	message.classList.add("chat-message")
	username.classList.add("chat-username")
	message_div.classList.add("message-div")



	message_div.appendChild(username)
	message_div.appendChild(message)
	CHAT_MESSAGES.prepend(message_div)
}

export function createServerMessage(text: string) {
	let message = document.createElement("div") as HTMLDivElement
	message.classList.add("server-message")
	message.innerText = text

	CHAT_MESSAGES.prepend(message)
}

export function sendMessage(event: Event){
	let input = event.target as HTMLInputElement
	input.value = input.value.trim()


	if (input.value != "") {
		console.log("Sending message: ", input.value)

		socket.emit("send-message", input.value)
	}


	input.value = ""
}

export function createChat() {
	if (CHAT_INPUT) {
		CHAT_INPUT.disabled = false
		CHAT_INPUT.placeholder = "Enter text"


		CHAT_INPUT.addEventListener("keypress", (event) => {
			if (event.code === "Enter") {
				sendMessage(event)

				ROOT.focus()
				event.stopPropagation()
			}
		})

		ROOT.addEventListener("keypress", (event) => {
			if (event.code === "Enter") {
				CHAT_INPUT.focus()
				stopInputs()
			}
		})
	}
}

export function createStartSceneChat() {
	if (CHAT_INPUT) {
		CHAT_INPUT.disabled = true
		CHAT_INPUT.placeholder = "Enter username to chat"
	}

	console.log("chat")
}

socket.on("receive-message", (data) => {
	createChatMessage(data)
	console.log(`Message from ${data.username}: ${data.message}`)
})


socket.on("join-global-chat", () => {

	createServerMessage("Joined global chat")
	console.log("Joined global chat")
})

socket.on("join-lobby-chat", () => {

	createServerMessage("Joined lobby chat")
	console.log("Joined lobby chat")
})

socket.on("join-game-chat", () => {

	createServerMessage("Joined game chat")
	console.log("Joined game chat")
})
