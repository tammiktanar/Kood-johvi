package session

import (
	"database/sql"
	"forum/internal/handler/query"
	"forum/internal/handler/structs"
	"net/http"
	"time"
)

var UserInfo structs.User

func Check(db *sql.DB, w http.ResponseWriter, r *http.Request) (bool, error) {
	cookie, err := r.Cookie("session")

	if err != nil {
		if err == http.ErrNoCookie { // If there isnt an existing cookie, there isnt an ongoing session
			UserInfo.Logged = false // 0 means no user is logged in
			return false, nil
		}

		return false, err

		// If cookie exists, get the UserID of the cookie and update the UserInfo.ID(which tracks, what is the logged in user ID)
	} else {
		// Check if that cookie belongs to user
		row := db.QueryRow("SELECT userid FROM sessions WHERE uuid = ?", cookie.Value)

		if err := row.Scan(&UserInfo.ID); err != nil {
			// If it wont find who the cookie belongs to - it deletes it
			if err == sql.ErrNoRows {
				cookie.Expires = time.Unix(0, 0)
				http.SetCookie(w, cookie)

				UserInfo.Logged = false // Resets the UserID if there is no ongoing session
				return false, nil       // Return nil because the error is handled
			}

			return false, err
		}

		// Get the logged in user's Username
		username, err := query.GetUsername(db, UserInfo.ID)
		if err != nil {
			http.Error(w, err.Error(), 500)
			return false, err
		}
		UserInfo.Username = username
		UserInfo.Logged = true

		return true, err
	}
}
