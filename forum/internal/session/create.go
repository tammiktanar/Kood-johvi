package session

import (
	"database/sql"
	"net/http"
	"time"

	"github.com/google/uuid"
)

func Create(userid int, w http.ResponseWriter, r *http.Request, db *sql.DB) {
	cookie, err := r.Cookie("session")

	if err != nil {
		if err != http.ErrNoCookie {
			http.Error(w, err.Error(), 500)
			return
		}
	}
	uuid := uuid.New().String()
	timeNow := time.Now()
	cookie = &http.Cookie{
		Name:    "session",
		Value:   uuid,
		Expires: timeNow.Add(time.Minute * 30),
	}

	http.SetCookie(w, cookie)
	if err := AddSession(db, userid, uuid, timeNow); err != nil {
		http.Error(w, err.Error(), 500) // Adding session to db
		return
	}

}

func AddSession(db *sql.DB, userid int, uuid string, timeNow time.Time) error {

	stmt, err := db.Prepare("DELETE FROM sessions WHERE userid = ?")
	if err == nil { // We only execute the statement if we find matches
		stmt.Exec(userid)
	} else if err != sql.ErrNoRows { // If the error is not ErrNoRows, something unexpected happened
		return err
	}

	// Adding the session into db
	stmt, err = db.Prepare("INSERT INTO sessions (userid, uuid, creation_date) VALUES (?, ?, ?)")
	if err != nil {
		return err
	}
	stmt.Exec(userid, uuid, timeNow.Format(time.ANSIC))

	return nil

}
