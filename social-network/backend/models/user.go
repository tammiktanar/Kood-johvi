package models

import (
	"database/sql"
	"fmt"
	"social-network/queries"
	"time"
)

type User struct {
	UserID int64 `json:"userID"`

	Email    string `json:"email"`
	Password string `json:"-"`

	FirstName string `json:"firstName"`
	LastName  string `json:"lastName"`
	Nickname  string `json:"nickname"`

	Created time.Time `json:"created"`

	Image    *string   `json:"image"`
	About    string    `json:"about"`
	Birthday time.Time `json:"birthday"`

	Private bool `json:"private"`

	FollowInfo *FollowInfo `json:"followInfo"`
}

type UserIncoming struct {
	Email    *string `json:"email"`
	Password *string `json:"password"`

	FirstName *string `json:"firstName"`
	LastName  *string `json:"lastName"`
	Nickname  *string `json:"nickname"`

	Image    *string    `json:"image"`
	About    *string    `json:"about"`
	Birthday *time.Time `json:"birthday"`

	Private bool `json:"private"`
}

type UserLimited struct {
	UserID     int64       `json:"userID"`
	FirstName  string      `json:"firstName"`
	LastName   string      `json:"lastName"`
	Nickname   string      `json:"nickname"`
	Image      *string     `json:"image"`
	FollowInfo *FollowInfo `json:"followInfo,omitempty"`
}

type FollowInfo struct {
	MeToYou        bool `json:"meToYou"`
	MeToYouPending bool `json:"meToYouPending"`
	YouToMePending bool `json:"youToMePending"`
}

func (x *User) pointerSlice() []interface{} {
	return []interface{}{
		&x.UserID,
		&x.Email,
		&x.Password,
		&x.FirstName,
		&x.LastName,
		&x.Nickname,
		&x.Created,
		&x.Image,
		&x.About,
		&x.Birthday,
		&x.Private,
	}
}

func (x *User) Limited() *UserLimited {
	return &UserLimited{
		UserID:     x.UserID,
		FirstName:  x.FirstName,
		LastName:   x.LastName,
		Nickname:   x.Nickname,
		Image:      x.Image,
		FollowInfo: x.FollowInfo,
	}
}

func (x *UserIncoming) pointerSlice() []interface{} {
	return []interface{}{
		x.Email,
		x.Password,
		x.FirstName,
		x.LastName,
		x.Nickname,
		x.Image,
		x.About,
		x.Birthday,
		x.Private,
	}
}

func (x *User) Censor(doIt bool) interface{} {
	if !doIt {
		return x
	}

	return &struct {
		UserID    int64  `json:"userID"`
		FirstName string `json:"firstName"`
		LastName  string `json:"lastName"`
		Nickname  string `json:"nickname"`
	}{
		UserID:    x.UserID,
		FirstName: x.FirstName,
		LastName:  x.LastName,
		Nickname:  x.Nickname,
	}
}

type UserModel struct {
	queries queries.QueryProvider
	db      *sql.DB
}

func MakeUserModel(db *sql.DB) UserModel {
	return UserModel{
		queries: queries.NewQueryProvider(db, "user"),
		db:      db,
	}
}

func (model *UserModel) GetByID(id int64) (*User, error) {
	stmt := model.queries.Prepare("getByID")

	row := stmt.QueryRow(id)

	user := &User{}
	err := row.Scan(user.pointerSlice()...)

	if err != nil {
		return nil, fmt.Errorf("User/GetByID: %w", err)
	}

	return user, nil
}

func (model *UserModel) GetByIDPlusFollowInfo(id int64, myID int64) (*User, error) {
	stmt := model.queries.Prepare("getByIDPlusFollowInfo")

	row := stmt.QueryRow(id, myID)

	user := &User{}
	followInfo := &FollowInfo{}
	user.FollowInfo = followInfo

	err := row.Scan(append(user.pointerSlice(), &followInfo.MeToYou, &followInfo.MeToYouPending, &followInfo.YouToMePending)...)

	if err != nil {
		return nil, fmt.Errorf("User/GetByIDPlusFollowInfo: %w", err)
	}

	return user, nil
}

func (model *UserModel) GetByEmail(email string) (*User, error) {
	stmt := model.queries.Prepare("getByEmail")

	row := stmt.QueryRow(email)

	user := &User{}
	err := row.Scan(user.pointerSlice()...)

	if err != nil {
		return nil, fmt.Errorf("User/GetByEmail: %w", err)
	}

	return user, nil
}

func (model *UserModel) Insert(user UserIncoming) (int64, error) {
	stmt := model.queries.Prepare("insert")

	res, err := stmt.Exec(
		user.pointerSlice()...,
	)

	if err != nil {
		return 0, fmt.Errorf("User/Insert: %w", err)
	}

	return res.LastInsertId()
}

func (model *UserModel) Update(id int64, user UserIncoming) error {
	stmt := model.queries.Prepare("update")

	_, err := stmt.Exec(
		append(user.pointerSlice(), id)...,
	)

	if err != nil {
		return fmt.Errorf("User/Update: %w", err)
	}

	return nil
}

func (model *UserModel) IsFollowing(followerID, followingID int64) (bool, error) {
	stmt := model.queries.Prepare("isFollowing")

	row := stmt.QueryRow(followerID, followingID)

	var includes bool
	err := row.Scan(&includes)

	if err != nil {
		return false, fmt.Errorf("User/IsFollowing: %w", err)
	}

	return includes, nil
}

func (model *UserModel) Follow(myID, targetID int64) error {
	stmt := model.queries.Prepare("follow")

	_, err := stmt.Exec(myID, targetID)

	if err != nil {
		return fmt.Errorf("User/Follow: %w", err)
	}

	return nil
}

func (model *UserModel) Unfollow(myID, targetID int64) error {
	stmt := model.queries.Prepare("unfollow")

	_, err := stmt.Exec(myID, targetID)

	if err != nil {
		return fmt.Errorf("User/Unfollow: %w", err)
	}

	return nil
}

func (model *UserModel) RequestFollow(myID, targetID int64) error {
	stmt := model.queries.Prepare("requestFollow")

	_, err := stmt.Exec(myID, targetID)

	if err != nil {
		return fmt.Errorf("User/RequestFollow: %w", err)
	}

	return nil
}

// TODO: Check if this should return an error if there was no request to accept
func (model *UserModel) FollowAccept(myID, targetID int64) error {
	stmt := model.queries.Prepare("followAccept")

	_, err := stmt.Exec(myID, targetID)

	if err != nil {
		return fmt.Errorf("User/FollowAccept: %w", err)
	}

	return nil
}

func (model *UserModel) ListFollowers(userID int64) ([]*UserLimited, error) {
	stmt := model.queries.Prepare("listFollowers")

	rows, err := stmt.Query(userID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("User/ListFollowers: %w", err)
	}

	users := make([]*UserLimited, 0)

	for rows.Next() {
		user := &User{}

		err = rows.Scan(user.pointerSlice()...)
		if err != nil {
			return nil, fmt.Errorf("User/ListFollowers: %w", err)
		}

		users = append(users, user.Limited())
	}

	return users, nil
}

func (model *UserModel) ListFollowing(userID int64) ([]*UserLimited, error) {
	stmt := model.queries.Prepare("listFollowing")

	rows, err := stmt.Query(userID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("User/ListFollowing: %w", err)
	}

	users := make([]*UserLimited, 0)

	for rows.Next() {
		user := &User{}

		err = rows.Scan(user.pointerSlice()...)
		if err != nil {
			return nil, fmt.Errorf("User/ListFollowing: %w", err)
		}

		users = append(users, user.Limited())
	}

	return users, nil
}

func (model *UserModel) Known(myID int64) ([]*UserLimited, error) {
	stmt := model.queries.Prepare("known")

	rows, err := stmt.Query(myID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("User/ListFollowing: %w", err)
	}

	users := make([]*UserLimited, 0)

	for rows.Next() {
		user := &User{}

		err = rows.Scan(user.pointerSlice()...)
		if err != nil {
			return nil, fmt.Errorf("User/ListFollowing: %w", err)
		}

		users = append(users, user.Limited())
	}

	return users, nil
}
