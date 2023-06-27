package models

import (
	"database/sql"
	"fmt"
	"log"
	"social-network/queries"
	"time"
)

type Event struct {
	EventID  int64 `json:"eventID"`
	GroupID  int64 `json:"groupID"`
	AuthorID int64 `json:"authorID"`

	Title string    `json:"title"`
	About string    `json:"about"`
	Time  time.Time `json:"time"`

	Created time.Time `json:"created"`

	MyStatus *string `json:"myStatus,omitempty"`
}

type EventMembers struct {
	Going    []*UserLimited `json:"going"`
	NotGoing []*UserLimited `json:"notGoing"`
}

func (x *Event) pointerSlice() []interface{} {
	return []interface{}{
		&x.EventID,
		&x.GroupID,
		&x.AuthorID,
		&x.Title,
		&x.About,
		&x.Time,
		&x.Created,
	}
}

type EventModel struct {
	queries queries.QueryProvider
	db      *sql.DB
}

func MakeEventModel(db *sql.DB) EventModel {
	return EventModel{
		queries: queries.NewQueryProvider(db, "event"),
		db:      db,
	}
}

func (model EventModel) GetByID(eventID, myID int64) (*Event, error) {
	stmt := model.queries.Prepare("getByID")

	row := stmt.QueryRow(eventID, myID)

	event := &Event{}

	err := row.Scan(append(event.pointerSlice(), &event.MyStatus)...)

	if err != nil {
		return nil, fmt.Errorf("Event/GetByID: %w", err)
	}

	return event, nil
}

func (model EventModel) GetByGroup(groupID, myID int64) ([]*Event, error) {
	stmt := model.queries.Prepare("getByGroup")

	rows, err := stmt.Query(groupID, myID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Event/GetByGroup: %w", err)
	}

	events := make([]*Event, 0)

	for rows.Next() {
		event := &Event{}

		err = rows.Scan(append(event.pointerSlice(), &event.MyStatus)...)
		if err != nil {
			return nil, fmt.Errorf("Event/GetByGroup: %w", err)
		}

		events = append(events, event)
	}

	return events, nil
}

func (model EventModel) GetByUser(userID int64) ([]*Event, error) {
	stmt := model.queries.Prepare("getByUser")

	rows, err := stmt.Query(userID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Event/GetByUser: %w", err)
	}

	events := make([]*Event, 0)

	for rows.Next() {
		event := &Event{}

		err = rows.Scan(event.pointerSlice()...)
		if err != nil {
			return nil, fmt.Errorf("Event/GetByUser: %w", err)
		}

		events = append(events, event)
	}

	return events, nil
}

func (model EventModel) GetMembers(groupID int64) (*EventMembers, error) {
	stmt := model.queries.Prepare("getMembers")

	rows, err := stmt.Query(groupID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Event/GetMembers: %w", err)
	}

	members := &EventMembers{
		Going:    make([]*UserLimited, 0),
		NotGoing: make([]*UserLimited, 0),
	}

	for rows.Next() {
		member := &User{}
		var status string

		err = rows.Scan(append(member.pointerSlice(), &status)...)
		if err != nil {
			return nil, fmt.Errorf("Event/GetMembers: %w", err)
		}

		switch status {
		case "GOING":
			members.Going = append(members.Going, member.Limited())
		case "NOT_GOING":
			members.NotGoing = append(members.NotGoing, member.Limited())
		default:
			log.Panicf("Event/GetMembers: Invalid going status: %v\n", status)
		}
	}

	return members, nil
}

func (model EventModel) Insert(group Event) (int64, error) {
	stmt := model.queries.Prepare("insert")

	res, err := stmt.Exec(
		group.pointerSlice()[:6]...,
	)

	if err != nil {
		return 0, fmt.Errorf("Event/Insert: %w", err)
	}

	return res.LastInsertId()
}

func (model EventModel) Going(eventID, userID int64) error {
	stmt := model.queries.Prepare("going")

	_, err := stmt.Exec(eventID, userID)

	if err != nil {
		return fmt.Errorf("Event/Going: %w", err)
	}

	return nil
}

func (model EventModel) NotGoing(eventID, userID int64) error {
	stmt := model.queries.Prepare("notGoing")

	_, err := stmt.Exec(eventID, userID)

	if err != nil {
		return fmt.Errorf("Event/NotGoing: %w", err)
	}

	return nil
}

func (model EventModel) Unset(eventID, userID int64) error {
	stmt := model.queries.Prepare("unset")

	_, err := stmt.Exec(eventID, userID)

	if err != nil {
		return fmt.Errorf("Event/Unset: %w", err)
	}

	return nil
}

func (model EventModel) HasAccess(eventID, userID int64) (bool, error) {
	stmt := model.queries.Prepare("hasAccess")

	row := stmt.QueryRow(eventID, userID)

	var result bool
	err := row.Scan(&result)

	if err != nil {
		return false, fmt.Errorf("Event/HasAccess: %w", err)
	}

	return result, nil
}

func (model EventModel) CanJoin(eventID, userID int64) (bool, error) {
	stmt := model.queries.Prepare("canJoin")

	row := stmt.QueryRow(eventID, userID)

	var result bool
	err := row.Scan(&result)

	if err != nil {
		return false, fmt.Errorf("Event/CanJoin: %w", err)
	}

	return result, nil
}
