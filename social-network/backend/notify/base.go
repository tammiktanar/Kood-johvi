package notify

import (
	"bytes"
	"encoding/json"
	"fmt"
	"github.com/gorilla/websocket"
	"html"
	"log"
	"net/http"
	"os"
	"social-network/database"
	"social-network/models"
	"time"
)

var frontend_host = getFrontendHost()

type Notification interface {
	Targets() []int64
	Message() string
	Links() []Link
}

type Link struct {
	name   string
	url    string
	method string
}

func (l Link) String() string {
	return fmt.Sprintf(
		"\n<button type=\"submit\" formmethod=\"%v\" formaction=\"%v\">%v</button>",
		html.EscapeString(l.method),
		html.EscapeString(l.url),
		html.EscapeString(l.name),
	)
}

type Notifier struct {
	channel  <-chan Notification
	upgrader websocket.Upgrader
	database *database.Database
}

func NewNotifier(db *database.Database) *Notifier {
	channel := make(chan Notification, 10)
	upgrader := websocket.Upgrader{
		ReadBufferSize:  1024,
		WriteBufferSize: 1024,
		CheckOrigin:     func(r *http.Request) bool { return true },
	}

	return &Notifier{
		channel:  channel,
		upgrader: upgrader,
		database: db,
	}
}

func (n Notifier) notify(msg Notification) {
	content := fmt.Sprintf("<span>%v</span>", msg.Message())
	content += "\n<form style='display: flex; flex-direction: column; gap: 2px; margin-top: 3px'>"
	for _, link := range msg.Links() {
		content += link.String()
	}
	content += "\n</form>"

	message := &models.Message{
		Sender:   0,
		Receiver: 0,
		Content:  content,
		Created:  time.Now(),
	}

	targets := msg.Targets()
	for _, t := range targets {
		message.Receiver = t
		_, err := n.database.Message.SendMessage(*message)
		if err != nil {
			log.Printf("could insert notification message for %v: %v\n", t, err)
		}
	}

	payload := struct {
		Targets []int64         `json:"targets"`
		Message *models.Message `json:"message"`
	}{
		Targets: targets,
		Message: message,
	}

	b := new(bytes.Buffer)
	err := json.NewEncoder(b).Encode(payload)
	if err != nil {
		log.Println(err)
	}

	_, err = http.Post(fmt.Sprintf("http://%v:8080/notify", frontend_host), "", b)
	if err != nil {
		log.Printf("could not notify notification: %v\n", err)
	}
}

func userGetName(u *models.User) string {
	if len(u.Nickname) > 0 {
		return u.Nickname
	}
	return fmt.Sprintf("%v %v", u.FirstName, u.LastName)
}

func conditionalString(b bool, s string) string {
	if b {
		return s
	}

	return ""
}

func getFrontendHost() string {
	v := os.Getenv("FRONTEND_ADDRESS")
	if v == "" {
		v = "localhost"
	}
	return v
}
