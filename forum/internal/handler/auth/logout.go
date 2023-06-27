package auth

import (
	"forum/internal/env"
	"net/http"
	"time"
)

func Logout(env *env.Env) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		cookie, err := r.Cookie("session")
		if err != nil {
			if err != http.ErrNoCookie {
				http.Error(w, err.Error(), 500)
				return
			}
		}

		db := env.DB
		stmt, err := db.Prepare("DELETE FROM sessions WHERE uuid = ?")
		if err != nil {
			http.Error(w, err.Error(), 500)
			return
		}
		stmt.Exec(cookie.Value)

		cookie.Expires = time.Unix(0, 0)
		http.SetCookie(w, cookie)

		http.Redirect(w, r, "/", 302)
		return

	}

}
