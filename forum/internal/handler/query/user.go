package query

import (
	"database/sql"
	"net/http"
)

func GetUserID(db *sql.DB, r *http.Request) (int, error) {
	cookie, err := r.Cookie("session")
	if err != nil {
		return 0, err
	}
	row := db.QueryRow("SELECT userid FROM sessions WHERE uuid = ?", cookie.Value)

	var userid int
	if err := row.Scan(&userid); err != nil {
		return 0, err
	}

	return userid, nil
}

func GetUsername(db *sql.DB, userid int) (string, error) {
	var username string
	if err := db.QueryRow("SELECT username FROM users WHERE id = ?", userid).Scan(&username); err != nil {
		return "", err
	}

	return username, nil
}
