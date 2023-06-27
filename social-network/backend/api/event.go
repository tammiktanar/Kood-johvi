package api

import (
	"encoding/json"
	"log"
	"net/http"
	"social-network/models"
	"social-network/router"
	"strconv"
	"time"
)

// rtr.Post("/api/event/create", api.EnsureAuth(api.CreateEvent))
func CreateEvent(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	event := &models.Event{}

	err := json.NewDecoder(r.Body).Decode(&event)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	event.AuthorID = session.UserID

	group, err := Database.Group.GetByID(event.GroupID, session.UserID)
	panicIfErr(err)
	if !group.IncludesMe {
		log.Printf("CreateEvent: User %v does not have access to group %v\n", session.UserID, event.GroupID)
		writeStatusError(w, http.StatusForbidden)
		return
	}

	id, err := Database.Event.Insert(*event)
	panicIfErr(err)

	event.EventID = id
	event.Created = time.Now()

	writeJSON(w, event)

	go func() {
		creator, err := Database.User.GetByID(session.UserID)
		if err != nil {
			log.Println(err)
		}

		members, err := Database.Group.GetMembers(group.GroupID)
		if err != nil {
			log.Println(err)
		}

		Notify.EventCreated(group.Group, event, creator, members)
	}()
}

// rtr.Post("/api/event/([0-9]+)/going", api.EnsureAuth(api.EventGoing))
func EventGoing(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)
	eventID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)

	access, err := Database.Event.CanJoin(eventID, session.UserID)
	panicIfErr(err)
	if !access {
		log.Printf("EventGoing: User %v is not part of event %v's group\n", session.UserID, eventID)
		writeStatusError(w, http.StatusForbidden)
		return
	}

	err = Database.Event.Going(eventID, session.UserID)
	panicIfErr(err)
}

// rtr.Post("/api/event/([0-9]+)/not-going", api.EnsureAuth(api.EventNotGoing))
func EventNotGoing(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)
	eventID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)

	access, err := Database.Event.CanJoin(eventID, session.UserID)
	panicIfErr(err)
	if !access {
		log.Printf("EventNotGoing: User %v is not part of event %v's group\n", session.UserID, eventID)
		writeStatusError(w, http.StatusForbidden)
		return
	}

	err = Database.Event.NotGoing(eventID, session.UserID)
	panicIfErr(err)
}

// rtr.Post("/api/event/([0-9]+)/unset", api.EnsureAuth(api.EventUnset))
func EventUnset(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)
	eventID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)

	err := Database.Event.Unset(eventID, session.UserID)
	panicIfErr(err)
}

// rtr.Get("/api/event/([0-9]+)", api.EventAccessCheck(api.GetEvent))
func GetEvent(w http.ResponseWriter, r *http.Request) {
	myID := getPossibleUserID(r)
	eventID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)
	members, err := Database.Event.GetByID(eventID, myID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, members)
}

// rtr.Get("/api/group/([0-9]+)/events", api.GroupAccessCheck(api.GetGroupEvents))
func GetGroupEvents(w http.ResponseWriter, r *http.Request) {
	myID := getPossibleUserID(r)
	groupID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)
	members, err := Database.Event.GetByGroup(groupID, myID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, members)
}

// rtr.Get("/api/event/([0-9]+)/members", api.EventAccessCheck(api.GetEventMembers))
func GetEventMembers(w http.ResponseWriter, r *http.Request) {
	eventID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)
	members, err := Database.Event.GetMembers(eventID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, members)
}

// rtr.Get("/api/event/all", api.EnsureAuth(api.GetMyEvents))
func GetMyEvents(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	members, err := Database.Event.GetByUser(session.UserID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, members)
}
