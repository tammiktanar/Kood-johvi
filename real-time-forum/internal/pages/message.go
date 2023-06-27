package pages

import (
	"encoding/json"
	"fmt"
	"forum/internal/forumDB"
	"forum/internal/forumEnv"
	"log"
	"net/http"
	"strings"
	"sync"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
}

var pool = &OnlinePool{
	users: make(map[int][]chan Msg),
}

type Message struct {
	forumEnv.Env
}

type messageData struct {
	forumEnv.GenericData
}

func (env Message) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	data := messageData{}
	data.InitData(env.Env, r)

	myUserID := data.User.UserID

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println(err)
		return
	}

	_ = conn.WriteMessage(1, []byte(fmt.Sprintf("{\"kind\": %v, \"content\": {\"id\": %v}}", USER_ID, myUserID)))

	if myUserID == 0 {
		_ = conn.Close()
		return
	}

	// fmt.Println("Socket opened")

	replyChan, isNew := pool.add(myUserID)

	// Notify other users of the new user joining
	userAsJSON, _ := json.Marshal(data.User)
	if isNew {
		pool.sendToAll(myUserID, Msg{
			Kind:    SERVER_USER_ONLINE,
			Content: userAsJSON,
		})
	}

	go messageSender(conn, replyChan)

	for {
		var msg Msg

		_, b, err := conn.ReadMessage()
		if err != nil {
			log.Println(fmt.Errorf("error receiving message: %w", err))
			break
		}

		err = json.Unmarshal(b, &msg)
		if err != nil {
			log.Println(fmt.Errorf("error unpacking message: %w", err))
			continue
		}

		env.handleIncoming(msg, replyChan, myUserID)
	}

	// Remove the user
	isLast := pool.remove(myUserID, replyChan)

	// Notify other users of the user leaving
	if isLast {
		pool.sendToAll(myUserID, Msg{
			Kind:    SERVER_USER_OFFLINE,
			Content: userAsJSON,
		})
	}

	_ = conn.Close()
	// fmt.Println("Socket closed")
}

func messageSender(conn *websocket.Conn, ch <-chan Msg) {
	writeMutex := sync.Mutex{}
	for msg := range ch {
		writeMutex.Lock()
		err := conn.WriteJSON(msg)
		if err != nil {
			log.Println(err)
		}
		writeMutex.Unlock()
	}
}

const (
	DEBUG   = 0
	USER_ID = 1

	CLIENT_REQUEST_ONLINE  = 11
	CLIENT_REQUEST_KNOWN   = 12
	CLIENT_USER_STATE      = 13
	CLIENT_SEND_MESSAGE    = 16
	CLIENT_REQUEST_HISTORY = 17
	CLIENT_START_TYPING    = 18

	SERVER_SEND_ONLINE     = 101
	SERVER_SEND_KNOWN      = 102
	SERVER_USER_STATE      = 103
	SERVER_USER_ONLINE     = 104
	SERVER_USER_OFFLINE    = 105
	SERVER_RECEIVE_MESSAGE = 106
	SERVER_SEND_HISTORY    = 107
	SERVER_IS_TYPING       = 108
)

func (env Message) handleIncoming(msg Msg, replyChan chan<- Msg, myUserID int) {
	switch msg.Kind {
	default:
		fallthrough
	case DEBUG:
		fmt.Printf("Echoing message: {Kind: %v, Text: %v}\n", msg.Kind, string(msg.Content))
		replyChan <- msg

	case CLIENT_REQUEST_ONLINE:
		// Request online users
		online := pool.getOnlineUsers(env.Env)
		content, err := json.Marshal(online)
		if err != nil {
			log.Println(fmt.Errorf("request online users: %w", err))
			return
		}

		replyChan <- Msg{
			Kind:    SERVER_SEND_ONLINE,
			Content: content,
		}

	case CLIENT_REQUEST_KNOWN:
		// Request known users
		known, err := env.Messages.KnownUsers(myUserID)
		if err != nil {
			log.Println(fmt.Errorf("request known users: %w", err))
			return
		}

		type onlineUser struct {
			Online bool         `json:"online"`
			User   forumDB.User `json:"user"`
		}

		payload := make([]onlineUser, len(known))
		for i, usr := range known {
			payload[i].Online = pool.isOnline(usr.UserID)
			payload[i].User = usr
		}

		content, err := json.Marshal(payload)
		if err != nil {
			log.Println(fmt.Errorf("request known users: %w", err))
			return
		}

		replyChan <- Msg{
			Kind:    SERVER_SEND_KNOWN,
			Content: content,
		}

	case CLIENT_USER_STATE:
		// Request user online state
		payloadIn := struct {
			Name string `json:"name,omitempty"`
			ID   int    `json:"id,omitempty"`
		}{}

		err := json.Unmarshal(msg.Content, &payloadIn)
		if err != nil {
			log.Println(fmt.Errorf("request user online state: %w", err))
			return
		}

		var target forumDB.User
		switch {
		case payloadIn.ID > 0:
			target, err = env.Users.Get(payloadIn.ID)
			if err != nil {
				log.Println(fmt.Errorf("request user online state: %w", err))
				return
			}

		case payloadIn.Name != "":
			target, err = env.Users.GetByName(strings.Title(strings.ToLower(payloadIn.Name)))
			if err != nil {
				log.Println(fmt.Errorf("request user online state: %w", err))
				return
			}

		default:
			log.Println("request user online state: invalid request")
			return
		}

		payloadOut := struct {
			Online bool         `json:"online"`
			User   forumDB.User `json:"user"`
		}{
			pool.isOnline(target.UserID),
			target,
		}

		contentOut, err := json.Marshal(payloadOut)
		if err != nil {
			log.Println(fmt.Errorf("request user online state: %w", err))
			return
		}

		replyChan <- Msg{
			Kind:    SERVER_USER_STATE,
			Content: contentOut,
		}

	case CLIENT_SEND_MESSAGE:
		// Send message
		payloadIn := struct {
			Receiver int    `json:"receiver,omitempty"`
			Text     string `json:"text,omitempty"`
		}{}

		err := json.Unmarshal(msg.Content, &payloadIn)
		if err != nil {
			log.Println(fmt.Errorf("send message: %w", err))
			return
		}

		message, err := env.Messages.Send(myUserID, payloadIn.Receiver, payloadIn.Text)
		if err != nil {
			log.Println(fmt.Errorf("send message: %w", err))
			return
		}

		contentOut, err := json.Marshal(message)
		if err != nil {
			log.Println(fmt.Errorf("send message: %w", err))
			return
		}

		msgOut := Msg{
			Kind:    SERVER_RECEIVE_MESSAGE,
			Content: contentOut,
		}

		pool.sendTo(payloadIn.Receiver, msgOut)

		pool.sendTo(myUserID, msgOut)

	case CLIENT_REQUEST_HISTORY:
		// Request history
		payloadIn := struct {
			Receiver    int `json:"receiver,omitempty"`
			FromMessage int `json:"from_message,omitempty"`
		}{}

		err := json.Unmarshal(msg.Content, &payloadIn)
		if err != nil {
			log.Println(fmt.Errorf("request history: %w", err))
			return
		}

		history, err := env.Messages.GetHistory(myUserID, payloadIn.Receiver, payloadIn.FromMessage)
		if err != nil {
			log.Println(fmt.Errorf("request history: %w", err))
			return
		}

		payloadOut := struct {
			UserID  int               `json:"user_id"`
			History []forumDB.Message `json:"history"`
		}{
			UserID:  payloadIn.Receiver,
			History: history,
		}

		contentOut, err := json.Marshal(payloadOut)
		if err != nil {
			log.Println(fmt.Errorf("request history: %w", err))
			return
		}

		replyChan <- Msg{
			Kind:    SERVER_SEND_HISTORY,
			Content: contentOut,
		}

	case CLIENT_START_TYPING:
		// Start typing
		payloadIn := struct {
			Receiver int `json:"receiver,omitempty"`
		}{}

		err := json.Unmarshal(msg.Content, &payloadIn)
		if err != nil {
			log.Println(fmt.Errorf("start typing: %w", err))
			return
		}

		payloadOut := struct {
			Sender int `json:"sender,omitempty"`
		}{
			myUserID,
		}

		contentOut, err := json.Marshal(payloadOut)
		if err != nil {
			log.Println(fmt.Errorf("start typing: %w", err))
			return
		}

		msgOut := Msg{
			Kind:    SERVER_IS_TYPING,
			Content: contentOut,
		}

		pool.sendTo(payloadIn.Receiver, msgOut)
	}
}

type Msg struct {
	Kind    int             `json:"kind"`
	Content json.RawMessage `json:"content"`
}

type OnlinePool struct {
	mutex sync.Mutex
	users map[int][]chan Msg

	dirty     bool
	onlineIDs []int
}

func (pool *OnlinePool) isOnline(userID int) bool {
	pool.mutex.Lock()
	defer pool.mutex.Unlock()

	_, online := pool.users[userID]
	return online
}

func (pool *OnlinePool) getOnlineUsers(env forumEnv.Env) []forumDB.User {
	var slice []int

	pool.mutex.Lock()

	if pool.dirty {
		slice = make([]int, len(pool.users))

		var i int
		for k := range pool.users {
			slice[i] = k
			i++
		}

		pool.onlineIDs = slice
		pool.dirty = false
	} else {
		slice = pool.onlineIDs
	}

	cp := make([]int, len(slice))
	copy(cp, slice)
	pool.mutex.Unlock()

	users := make([]forumDB.User, len(cp))
	for i, id := range cp {
		users[i], _ = env.Users.Get(id)
	}

	return users
}

func (pool *OnlinePool) add(user int) (chan Msg, bool) {
	pool.mutex.Lock()
	defer pool.mutex.Unlock()
	pool.dirty = true

	slc, found := pool.users[user]
	if !found {
		slc = make([]chan Msg, 0, 1)
	}

	ch := make(chan Msg, 1)
	pool.users[user] = append(slc, ch)

	return ch, !found
}

func (pool *OnlinePool) remove(user int, ch chan Msg) bool {
	pool.mutex.Lock()
	pool.dirty = true

	slc, found := pool.users[user]
	if !found {
		return false
	}

	for i, c := range slc {
		if c == ch {
			slc[i] = slc[len(slc)-1]
			pool.users[user] = slc[:len(slc)-1]

			close(ch)

			break
		}
	}

	last := false

	if len(pool.users[user]) == 0 {
		delete(pool.users, user)
		last = true
	}
	pool.mutex.Unlock()

	return last
}

func (pool *OnlinePool) sendToAll(except int, msg Msg) {
	pool.mutex.Lock()
	defer pool.mutex.Unlock()
	for id, slc := range pool.users {
		if id == except {
			continue
		}
		for _, ch := range slc {
			ch <- msg
		}
	}
}

func (pool *OnlinePool) sendTo(user int, msg Msg) {
	pool.mutex.Lock()
	defer pool.mutex.Unlock()

	slc := pool.users[user]
	for _, c := range slc {
		c <- msg
	}
}
