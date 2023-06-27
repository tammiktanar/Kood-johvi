package handler

import (
	"fmt"
	"forum/internal/env"
	"forum/internal/handler/auth"
	"forum/internal/handler/query"
	"forum/internal/session"
	"net/http"
	"time"
)

func AddComment(env *env.Env) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {

		db := env.DB                             // intializes db connection
		isLogged, err := session.Check(db, w, r) // checks if user is logged in

		// If an actual error happened in session.Check
		if err != nil {
			http.Error(w, err.Error(), 500)
			return
		}

		// If user not logged
		if !isLogged {
			http.Redirect(w, r, "/login", 302)
			auth.LoginMsgs.LoginRequired = true
			return
		}

		if r.Method != "POST" {
			http.Error(w, "Only POST request allowed", 400)
			return
		}

		if err := r.ParseForm(); err != nil {
			http.Error(w, err.Error(), 500)
			return
		}

		postid := r.URL.Query().Get("post") // id is the ID of the post, which we get from URL

		// CheckQuery checks if the id from URL is valid and exists
		if err := query.CheckURLQuery(db, "SELECT postid FROM posts WHERE postid = ?", postid); err != nil {
			http.Error(w, err.Error(), 400)
			return
		}

		userid, err := query.GetUserID(db, r)
		if err != nil {
			http.Error(w, err.Error(), 500)
			return
		}

		stmt, err := db.Prepare("INSERT INTO comments (body, postid, userid, creation_date) VALUES (?, ?, ?, ?)")
		if err != nil {
			http.Error(w, err.Error(), 500)
			return
		}

		timeNow := time.Now()
		stmt.Exec(r.FormValue("body"), postid, userid, timeNow.Format(time.ANSIC))

		redirectURL := fmt.Sprintf("/post?id=%v", postid) // Redirects user to the same page where he was after posting the comment
		http.Redirect(w, r, redirectURL, 302)

	}
}
