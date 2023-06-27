package models

import (
	"database/sql"
	"fmt"
	"social-network/queries"
	"time"
)

type Group struct {
	GroupID int64 `json:"groupID"`
	OwnerID int64 `json:"ownerID"`

	Name  string  `json:"name"`
	About string  `json:"about"`
	Image *string `json:"image"`
	Type  string  `json:"type"`

	Created time.Time `json:"created"`
}

type GroupPlus struct {
	*Group
	IncludesMe     bool `json:"includesMe"`
	PendingRequest bool `json:"pendingRequest"`
}

func (x *Group) pointerSlice() []interface{} {
	return []interface{}{
		&x.GroupID,
		&x.OwnerID,
		&x.Name,
		&x.About,
		&x.Image,
		&x.Type,
		&x.Created,
	}
}

type GroupModel struct {
	queries queries.QueryProvider
	db      *sql.DB
}

func MakeGroupModel(db *sql.DB) GroupModel {
	return GroupModel{
		queries: queries.NewQueryProvider(db, "group"),
		db:      db,
	}
}

func (model GroupModel) GetByID(groupID, myID int64) (*GroupPlus, error) {
	stmt := model.queries.Prepare("getByID")

	row := stmt.QueryRow(groupID, myID)

	group := &Group{}
	groupPlus := &GroupPlus{Group: group}

	err := row.Scan(append(group.pointerSlice(), &groupPlus.IncludesMe, &groupPlus.PendingRequest)...)

	if err != nil {
		return nil, fmt.Errorf("Group/GetByID: %w", err)
	}

	return groupPlus, nil
}

func (model GroupModel) GetAll(myID int64) ([]*GroupPlus, error) {
	stmt := model.queries.Prepare("getAll")

	rows, err := stmt.Query(myID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Group/GetAll: %w", err)
	}

	groups := make([]*GroupPlus, 0)

	for rows.Next() {
		group := &Group{}
		groupPlus := &GroupPlus{Group: group}

		err = rows.Scan(append(group.pointerSlice(), &groupPlus.IncludesMe, &groupPlus.PendingRequest)...)
		if err != nil {
			return nil, fmt.Errorf("Group/GetAll: %w", err)
		}

		groups = append(groups, groupPlus)
	}

	return groups, nil
}

func (model GroupModel) GetMyGroups(myID int64) ([]*Group, error) {
	stmt := model.queries.Prepare("getMyGroups")

	rows, err := stmt.Query(myID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Group/GetMyGroups: %w", err)
	}

	groups := make([]*Group, 0)

	for rows.Next() {
		group := &Group{}

		err = rows.Scan(group.pointerSlice()...)
		if err != nil {
			return nil, fmt.Errorf("Group/GetMyGroups: %w", err)
		}

		groups = append(groups, group)
	}

	return groups, nil
}

func (model GroupModel) Insert(group Group) (int64, error) {
	stmt := model.queries.Prepare("insert")

	res, err := stmt.Exec(
		group.pointerSlice()[:6]...,
	)

	if err != nil {
		return 0, fmt.Errorf("Group/Insert: %w", err)
	}

	return res.LastInsertId()
}

func (model GroupModel) Join(groupID, userID int64) error {
	stmt := model.queries.Prepare("join")

	_, err := stmt.Exec(groupID, userID)

	if err != nil {
		return fmt.Errorf("Group/Join: %w", err)
	}

	return nil
}

func (model GroupModel) JoinCheck(groupID, userID int64) (bool, error) {
	stmt := model.queries.Prepare("joinCheck")

	row := stmt.QueryRow(groupID, userID)

	var result bool
	err := row.Scan(&result)

	if err != nil {
		return false, fmt.Errorf("Group/JoinCheck: %w", err)
	}

	return result, nil
}

func (model GroupModel) Request(groupID, userID int64) error {
	stmt := model.queries.Prepare("request")

	_, err := stmt.Exec(groupID, userID)

	if err != nil {
		return fmt.Errorf("Group/Request: %w", err)
	}

	return nil
}

func (model GroupModel) Leave(groupID, userID int64) error {
	stmt := model.queries.Prepare("leave")

	_, err := stmt.Exec(groupID, userID)

	if err != nil {
		return fmt.Errorf("Group/Leave: %w", err)
	}

	return nil
}

func (model GroupModel) HasAccess(groupID, userID int64) (bool, error) {
	stmt := model.queries.Prepare("hasAccess")

	row := stmt.QueryRow(groupID, userID)

	var result bool
	err := row.Scan(&result)

	if err != nil {
		return false, fmt.Errorf("Group/HasAccess: %w", err)
	}

	return result, nil
}

func (model GroupModel) GetMembers(groupID int64) ([]*User, error) {
	stmt := model.queries.Prepare("getMembers")

	rows, err := stmt.Query(groupID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Group/GetMembers: %w", err)
	}

	users := make([]*User, 0)

	for rows.Next() {
		user := &User{}

		err = rows.Scan(user.pointerSlice()...)
		if err != nil {
			return nil, fmt.Errorf("Group/GetMembers: %w", err)
		}

		users = append(users, user)
	}

	return users, nil
}

func (model GroupModel) Invite(groupID, myID, userID int64) error {
	stmt := model.queries.Prepare("invite")

	_, err := stmt.Exec(groupID, myID, userID)

	if err != nil {
		return fmt.Errorf("Group/Invite: %w", err)
	}

	return nil
}

func (model GroupModel) InviteCheck(groupID, userID int64) (bool, error) {
	stmt := model.queries.Prepare("inviteCheck")

	row := stmt.QueryRow(groupID, userID)

	var result bool
	err := row.Scan(&result)

	if err != nil {
		return false, fmt.Errorf("Group/InviteCheck: %w", err)
	}

	return result, nil
}

func (model GroupModel) IncludesUser(groupID, userID int64) (bool, error) {
	stmt := model.queries.Prepare("includesUser")

	row := stmt.QueryRow(groupID, userID)

	var includes bool
	err := row.Scan(&includes)

	if err != nil {
		return false, fmt.Errorf("Group/IncludesUser: %w", err)
	}

	return includes, nil
}

func (model GroupModel) TransferOwnership(groupID, userID int64) error {
	stmt := model.queries.Prepare("transferOwnership")

	_, err := stmt.Exec(groupID, userID)

	if err != nil {
		return fmt.Errorf("Group/TransferOwnership: %w", err)
	}

	return nil
}

func (model GroupModel) GetPendingInvites(groupID int64) ([]int64, error) {
	stmt := model.queries.Prepare("getPendingInvites")

	rows, err := stmt.Query(groupID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Group/GetPendingInvites: %w", err)
	}

	users := make([]int64, 0)

	for rows.Next() {
		var id int64
		err = rows.Scan(&id)
		if err != nil {
			return nil, fmt.Errorf("Group/GetPendingInvites: %w", err)
		}

		users = append(users, id)
	}

	return users, nil
}
