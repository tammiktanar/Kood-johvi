package handler

import (
	"forum/internal/env"
	"forum/internal/handler/auth"
	"forum/internal/handler/query"
	"forum/internal/session"
	"net/http"
)

/*
	Reacting to a post or comment:
	Upon reacting it will sent to an url, where you will have a "like" and "post" or "commentid" query
*/

func React(env *env.Env) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		db := env.DB
		isLogged, err := session.Check(db, w, r)
		if err != nil {
			http.Error(w, err.Error(), 500)
			return
		}

		if !isLogged {
			http.Redirect(w, r, "/login", 302)
			auth.LoginMsgs.LoginRequired = true
			return
		}

		userid, err := query.GetUserID(db, r)
		if err != nil {
			http.Error(w, err.Error(), 500)
			return
		}

		var isLike int // 1 means user liked the post and 0 means user disliked

		like := r.URL.Query().Get("like") // Get the value of the "like" to see if user liked or disliked
		if like == "true" {
			isLike = 1
		}

		commentid := r.URL.Query().Get("comment")
		postid := r.URL.Query().Get("post")

		// User reacted to a comment because there is a commentid in url
		if commentid != "" {
			// CheckQuery checks if the id from URL is valid and exists
			if err := query.CheckURLQuery(db, "SELECT id FROM comments WHERE id = ?", commentid); err != nil {
				http.Error(w, err.Error(), 400)
				return
			}

			err = query.CheckCommentLikes(db, userid, commentid, isLike)
			if err != nil {
				http.Error(w, err.Error(), 500)
				return
			}

			// Get the postid of the comment so we can redirect user to the same post after liking a comment
			if err := db.QueryRow("SELECT postid FROM comments WHERE id = ?", commentid).Scan(&postid); err != nil {
				http.Error(w, err.Error(), 500)
				return
			}

			// User reacted to a post because there is a postid in url
		} else if postid != "" {
			// CheckQuery checks if the id from URL is valid and exists
			if err := query.CheckURLQuery(db, "SELECT postid FROM posts WHERE postid = ?", postid); err != nil {
				http.Error(w, err.Error(), 400)
				return
			}

			err = query.CheckPostLikes(db, postid, userid, isLike)
			if err != nil {
				http.Error(w, err.Error(), 500)
				return
			}

		}

		http.Redirect(w, r, "/post?id="+postid, 302)
		return

	}
}
