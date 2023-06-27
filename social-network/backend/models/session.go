package models

import (
	"database/sql"
	"fmt"
	"github.com/google/uuid"
	"social-network/queries"
	"time"
)

type Session struct {
	Token  string `json:"token"`
	UserID int64  `json:"userID"`

	Created time.Time `json:"created"`
	Expires time.Time `json:"expires"`
}

func (x *Session) pointerSlice() []interface{} {
	return []interface{}{
		&x.Token,
		&x.UserID,
		&x.Created,
		&x.Expires,
	}
}

type SessionModel struct {
	queries queries.QueryProvider
	db      *sql.DB
}

func MakeSessionModel(db *sql.DB) SessionModel {
	return SessionModel{
		queries: queries.NewQueryProvider(db, "session"),
		db:      db,
	}
}

func (model SessionModel) Get(token string) (*Session, error) {
	stmt := model.queries.Prepare("get")

	row := stmt.QueryRow(token)

	session := &Session{}
	err := row.Scan(session.pointerSlice()...)
	if err != nil {
		return nil, fmt.Errorf("Session/Auth: %w", err)
	}

	if time.Now().After(session.Expires) {
		_, _ = model.Delete(token)
		return nil, fmt.Errorf("Session/Auth: expired token: %w", sql.ErrNoRows)
	}

	return session, nil
}

func (model SessionModel) Insert(userID int64, duration time.Duration) (string, error) {
	stmt := model.queries.Prepare("insert")

	token := uuid.New().String()
	expire := time.Now().Add(duration)

	_, err := stmt.Exec(token, userID, expire)
	if err != nil {
		return "", fmt.Errorf("Session/Insert: %w", err)
	}

	return token, err
}

func (model SessionModel) Delete(token string) (bool, error) {
	stmt := model.queries.Prepare("delete")

	res, err := stmt.Exec(token)
	if err != nil {
		return false, fmt.Errorf("Session/ClearUser: %w", err)
	}

	n, _ := res.RowsAffected()

	return n > 0, nil
}

func (model SessionModel) ClearUser(userID int64) error {
	stmt := model.queries.Prepare("clearUser")

	_, err := stmt.Exec(userID)
	if err != nil {
		return fmt.Errorf("Session/ClearUser: %w", err)
	}

	return nil
}

func (model SessionModel) CleanExpired() (int64, error) {
	stmt := model.queries.Prepare("cleanExpired")

	res, err := stmt.Exec()
	if err != nil {
		return 0, fmt.Errorf("Session/CleanExpired: %w", err)
	}

	return res.RowsAffected()
}

func (model SessionModel) SetExpires(token string, duration time.Duration) (bool, error) {
	stmt := model.queries.Prepare("setExpires")

	res, err := stmt.Exec(time.Now().Add(duration), token)
	if err != nil {
		return false, fmt.Errorf("Session/SetExpires: %w", err)
	}

	n, _ := res.RowsAffected()

	return n > 0, nil
}
