export default class Chat {
    get element() {return this.#wrapper}

    /** @type HTMLElement */ #wrapper
    /** @type HTMLElement */ #hideButton

    /** @type HTMLElement */ #userList
    /** @type HTMLElement */ #usernameInput
    /** @type HTMLElement */ #knownList
    /** @type HTMLElement */ #onlineList

    /** @type HTMLElement */ #messageTab
    /** @type HTMLElement */ #messageUser
    /** @type HTMLElement */ #messageHistory
    /** @type HTMLElement */ #messageHistoryInner
    /** @type HTMLElement */ #messageInput

    /** @type IntersectionObserver */ #observer
    /** @type HTMLElement */ #historyTop

    /** @type HTMLElement */ #typing

    /** @type HTMLAudioElement */ #audio

    constructor() {
        // Init HTML template and get it in a variable
        const tempDiv = document.createElement("div")
        tempDiv.innerHTML = chatTemplate()
        this.#wrapper = tempDiv.firstElementChild


        // Find and set the rest of the variables
        this.#hideButton = this.#wrapper.querySelector("#chat-hide")

        this.#userList = this.#wrapper.querySelector("#chat-users")
        this.#usernameInput = this.#wrapper.querySelector("#chat-users-search")
        this.#knownList = this.#wrapper.querySelector("#chat-users-known")
        this.#onlineList = this.#wrapper.querySelector("#chat-users-online")

        this.#messageTab = this.#wrapper.querySelector("#chat-convo")
        this.#messageUser = this.#wrapper.querySelector("#chat-convo-user")
        this.#messageHistory = this.#wrapper.querySelector("#chat-convo-history")
        this.#messageHistoryInner = this.#messageHistory.firstElementChild
        this.#messageInput = this.#wrapper.querySelector("#chat-convo-input")

        this.#historyTop = this.#wrapper.querySelector("#chat-history-top")
        this.#observer = new IntersectionObserver(this.#observerHandler.bind(this), {
            root: this.#messageHistory
        })
        this.#observer.observe(this.#historyTop)

        this.#typing = this.#wrapper.querySelector("#chat-typing")

        this.#audio = new Audio("/notification.wav")


        // Add appropriate event listeners

        // Hide/show the chat
        this.#hideButton.onclick = () => {
            this.#wrapper.classList.toggle("hide")
        }

        // User searching
        this.#usernameInput.onkeydown = (e) => {
            if (e.code !== "Enter") {
                return
            }

            this.onSearchUser(this.#usernameInput.value)
        }

        // Message sending
        this.#messageInput.onkeydown = (e) => {
            if (e.code !== "Enter" || this.#messageInput.value === "") {
                return
            }

            this.onSendMessage(this.#messageInput.value, this.activeUser)
            this.#messageInput.value = ""
        }

        const throttled = customThrottle(() => this.onTyping(this.activeUser), 1000, {leading: true})
        this.#messageInput.oninput = (e) => {
            if (e.inputType === "deleteContentBackward") return

            throttled()
        }

        // Switching chat between users
        this.#userList.onclick = (e) => {
            if (!e.target.classList.contains("chat-user")) return

            /** @type HTMLElement */
            const user = e.target

            if (user.classList.contains("selected")) {
                // Clicked on already selected user, so we close
                this.activeUser = 0
                user.classList.remove("selected")
                this.#messageTab.classList.add("hide")
                return
            }

            // Deselect already selected user
            this.#userList.querySelectorAll(".selected")
                .forEach(elem => elem.classList.remove("selected"))

            // Select this user
            user.classList.add("selected")

            // Clear convo
            this.#clearMessages()

            // Unhide convo
            this.#messageTab.classList.remove("hide")

            // Set user in convo tab
            this.#messageUser.innerHTML = user.outerHTML

            this.activeUser = parseInt(user.dataset.userId)

            // Request message history
            this.resetHistoryCooldown()
        }
    }


    // Tries bumping an existing user to/in known users. If successful returns true, if not returns false.
    bumpKnownUser(userID) {
        const known = this.#userList.querySelector(`[data-user-id="${userID}"]`) ||
            this.#onlineList.querySelector(`[data-user-id="${userID}"]`)
        if (known !== null) {
            // known.remove()
            this.#knownList.insertAdjacentElement("afterbegin", known)
            return true
        }

        return false
    }


    /** If a user is known, bump it to the top of the list, otherwise add the user to the START of the known list.
     * @param {User} user
     * @param {boolean} isNewOnline
     */
    userKnown(user, isNewOnline = false) {
        const known = this.#userList.querySelector(`[data-user-id="${user.user_id}"]`) ||
                      this.#onlineList.querySelector(`[data-user-id="${user.user_id}"]`)
        if (known !== null) {
            // known.remove()
            this.#knownList.insertAdjacentElement("afterbegin", known)
            return
        }

        const elem = this.#knownList.insertAdjacentElement("afterbegin", createUserElement(user))
        if (isNewOnline)
            elem.classList.add("online")
    }

    /** If a user is known, make it online, otherwise add the user to the END of the online list.
     *  @param {User} user */
    userOnline(user) {
        this.#setActiveUserState(user, true)

        const known = this.#knownList.querySelector(`[data-user-id="${user.user_id}"]`)
        if (known !== null) {
            known.classList.add("online")
            return
        }

        const existing = this.#onlineList.querySelector(`[data-user-id="${user.user_id}]"`)
        if (existing === null) {
            const elem = this.#onlineList.appendChild(createUserElement(user))
            elem.classList.add("online")
            return
        }

        console.warn(`Attempted to set the online user ${user.user_id} online again.`)
    }

    /** If a user is known, make it offline, otherwise remove the user from the online list.
     *  @param {User} user */
    userOffline(user) {
        this.#setActiveUserState(user, false)

        const known = this.#knownList.querySelector(`[data-user-id="${user.user_id}"]`)
        if (known !== null) {
            known.classList.remove("online")
            return
        }

        const existing = this.#onlineList.querySelector(`[data-user-id="${user.user_id}"]`)
        if (existing !== null) {
            existing.remove()
            return
        }

        console.warn(`Attempted to set the offline user ${user.name} (${user.user_id}) offline again.`)
    }


    activeUser = 0

    #setActiveUserState(user, online) {
        if (user.user_id !== this.activeUser) return

        this.#messageUser.classList.toggle("online", online)
    }

    addNewMessage(incoming, text, date) {
        const message = createMessageElement(incoming, text, date)
        this.#messageHistoryInner.appendChild(message)

        if (incoming) this.#typing.style.display = "block"
    }

    addOldMessage(incoming, text, date, messageID) {
        const message = createMessageElement(incoming, text, date, messageID)
        this.#messageHistoryInner.insertAdjacentElement("afterbegin", message)
    }

    #clearMessages() {
        this.#messageHistoryInner.innerHTML = ""
        this.#messageHistoryInner = this.#messageHistory.firstElementChild

        this.#historyTop.style.display = "block"
    }


    onSearchUser = function (username) {}

    onSendMessage = function (text, receiverID) {}

    onLoadMessages = function (userID, fromMessage) {}

    onTyping = function (userID) {}



    #cooldown = 3000
    #waiting = false
    #timeoutID = 0

    /**
     * @param {IntersectionObserverEntry} entry
     * @param {IntersectionObserver} observer
     */
    #observerHandler([entry]) {
        // console.log("Observer triggered")
        if (this.#waiting || !entry.isIntersecting) return

        this.#waiting = true
        this.#timeoutID = setTimeout(() => this.#waiting = false, this.#cooldown)

        this.onLoadMessages(this.activeUser, parseInt(this.#messageHistoryInner.firstElementChild?.dataset.messageId) || 0)
    }

    resetHistoryCooldown() {
        this.#waiting = false
        clearTimeout(this.#timeoutID)

        this.#observer.unobserve(this.#historyTop)
        this.#observer.observe(this.#historyTop)
    }

    // Tell the chat that no more messages are coming as part of the current chat history
    noMoreHistory() {
        this.#historyTop.style.display = "none"
    }

    #typingTimeout = 0
    #typingDelay = 1000

    isTyping() {
        this.#typing.innerText = this.#wrapper.querySelector("#chat-convo-user .chat-username")?.innerText + " is typing..."
        this.#typing.style.display = "block"

        clearTimeout(this.#typingTimeout)
        this.#typingTimeout = setTimeout(() => {
            this.#typing.style.display = "none"
        }, this.#typingDelay)
    }

    playSound() {
        this.#audio.play().then()
    }
}

// language=HTML
const chatTemplate = () => `
	<div id="chat-wrapper" class="hide">
		<div id="chat-hide">
			<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
				<path d="M2 5a2 2 0 012-2h7a2 2 0 012 2v4a2 2 0 01-2 2H9l-3 3v-3H4a2 2 0 01-2-2V5z"/>
				<path d="M15 7v2a4 4 0 01-4 4H9.828l-1.766 1.767c.28.149.599.233.938.233h2l3 3v-3h2a2 2 0 002-2V9a2 2 0 00-2-2h-1z"/>
			</svg>
		</div>
		<div id="chat-users">
			<input type="text" id="chat-users-search" placeholder="Add user...">

			<div class="chat-user-group-title">Known Users</div>
			<div id="chat-users-known" class="chat-user-group"></div>

			<div class="chat-user-group-title">Online Users</div>
			<div id="chat-users-online" class="chat-user-group"></div>
		</div>

		<div id="chat-convo" class="hide">
			<div id="chat-convo-user"></div>

			<div id="chat-convo-history">
				<div></div>
                <div id="chat-history-top" style="text-align: center"></div>
			</div>

			<input type="text" id="chat-convo-input" placeholder="Send a message...">
            <div id="chat-typing" style="display: none;"></div>
		</div>
	</div>
`




/** A user
 * @typedef {Object} User
 * @property {number} user_id
 * @property {string} name
 * @property {string} image
 * @property {string} first_name
 * @property {string} last_name
 */

/** Creates a user HTML element out of a user object. Checks for missing keys as well.
 * @param {User} user
 * @return {HTMLElement}
 */
function createUserElement(user = {}) {
    // Setting defaults just in case
    applyUserDefaults(user, {
        user_id: 0,
        name: "ERROR",
        image: "0-0.png",
        first_name: "ERROR",
        last_name: "ERROR",
    })

    const tempDiv = document.createElement("div")
    tempDiv.innerHTML = userTemplate(user)
    return tempDiv.firstElementChild
}


/**
 *
 * @param {User} user
 * @return {string}
 */
const userTemplate = (user) => `
	<div class="chat-user" data-user-id="${user.user_id}">
		<img class="chat-user-image clickthrough" src="/profile-pictures/${user.image}" alt="">
		<div class="chat-user-names clickthrough">
			<span class="chat-username">${user.name}</span>
			<span class="chat-fullname">${user.first_name} ${user.last_name}</span>
		</div>
	</div>
`

function applyUserDefaults(options, defaults) {
    const keys = Object.keys(defaults)
    keys.forEach(key => {
        if (!(key in options)) {
            options[key] = defaults[key]
            console.error(`Provided user object didn't have the key "${key}", using "${defaults[key]}" as a default.`)
        }
    })
    return options
}

/**
 *
 * @param {boolean} incoming
 * @param {string} text
 * @param {string} date
 * @param {number} messageID
 * @return {HTMLElement}
 */
function createMessageElement(incoming, text, date, messageID) {
    const tempDiv = document.createElement("div")
    tempDiv.innerHTML = messageTemplate(incoming, text, date, messageID)

    const element = tempDiv.firstElementChild
    element.innerText = text

    return element
}

/**
 * @param {boolean} incoming
 * @param {string} text
 * @param {string} date
 * @param {number} messageID
 * @return {string}
 */
const messageTemplate = (incoming, text, date, messageID = 0) => `
	<div class="message ${incoming ? "in" : "out"}" data-message-id="${messageID}" title="${date}"></div>
`


function customThrottle(fn, delay, options) {
    let timeout = false
    let doTrailing = false
    return function (...args) {
        if (!timeout) {
            timeout = true
            setTimeout(() => {
                timeout = false

                if (options?.trailing && doTrailing)
                    fn(...args)
                doTrailing = false
            }, delay)

            if (options?.leading)
                fn(...args)
        } else {
            doTrailing = true
        }
    }
}
