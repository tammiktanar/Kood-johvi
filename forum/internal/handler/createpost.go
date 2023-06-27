package handler

import (
	"database/sql"
	"forum/internal/env"
	"forum/internal/handler/auth"
	"forum/internal/handler/query"
	"forum/internal/handler/structs"
	"forum/internal/session"
	"forum/internal/tpl"
	"net/http"
	"time"
)

// "createpost.html" uses "base" template, which has a navbar what uses data from UserInfo
type CreatePostPage struct {
	UserInfo structs.User
	Tags     []string
}

func CreatePost(env *env.Env) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		db := env.DB // intializes db connection

		isLogged, err := session.Check(db, w, r)
		if err != nil && err != sql.ErrNoRows {
			http.Error(w, err.Error(), 500)
			return
		}

		if !isLogged {
			http.Redirect(w, r, "/login", 302)
			auth.LoginMsgs.LoginRequired = true
			return
		}

		if r.Method == "POST" { // If user is creating a post

			if err := r.ParseForm(); err != nil {
				http.Error(w, err.Error(), 400)
				return
			}

			// Store all the post data to Posts table
			if err := addPost(db, r); err != nil {
				http.Error(w, err.Error(), 500)
				return
			}

			// Store the tags to Tags table
			if err := addTags(db, r); err != nil {
				http.Error(w, err.Error(), 500)
				return
			}

			// Add tags connected to post to PostTags table
			if err := addPostTags(db, r); err != nil {
				http.Error(w, err.Error(), 500)
				return
			}

			http.Redirect(w, r, "/", 302)
			return

		} else if r.Method == "GET" { // If the method is GET

			// allTags is for displaying all the possible tags while creating the post
			allTags, err := query.GetAllTags(db)
			if err != nil {
				http.Error(w, err.Error(), 500)
				return
			}
			createPostPage := CreatePostPage{
				UserInfo: session.UserInfo,
				Tags:     allTags,
			}

			tpl.RenderTemplates(w, "createpost.html", createPostPage, "./templates/createpost.html", "./templates/base.html")
			return

		} else {
			http.Error(w, "Wrong type of request", 400)
			return
		}

	}

}

/*
1. Get the ID of the user by using UUID from the cookie
2. Add the post title, body and ID of the user into Posts table
*/
func addPost(db *sql.DB, r *http.Request) error {
	// We use cookie to get the ID of the user who created the post

	userid, err := query.GetUserID(db, r)
	if err != nil {
		return err
	}

	// Add new post to database
	stmt, err := db.Prepare("INSERT INTO posts (title, body, userid, creation_date) VALUES (?, ?, ?, ?)")
	if err != nil {
		return err
	}

	timeNow := time.Now()
	stmt.Exec(r.FormValue("title"), r.FormValue("body"), userid, timeNow.Format(time.ANSIC))
	return nil

}

// Adds tag names to Tags table
func addTags(db *sql.DB, r *http.Request) error {

	stmt, err := db.Prepare("INSERT OR IGNORE INTO tags (name) VALUES (?)")
	if err != nil {
		return err
	}

	for _, tag := range r.Form["tags"] {
		stmt.Exec(tag)
	}

	return nil
}

/*
1. Get the ID of the tags
2. Get the ID of the post
3. Add all the used tag IDs and post ID into the PostTags table
*/
func addPostTags(db *sql.DB, r *http.Request) error {
	var tagIDs []string

	for _, tag := range r.Form["tags"] {

		var tagid string
		if err := db.QueryRow("SELECT id FROM tags WHERE name = ?", tag).Scan(&tagid); err != nil {
			return err
		}

		tagIDs = append(tagIDs, tagid)

	}

	row := db.QueryRow("SELECT postid FROM posts WHERE title = ?", r.FormValue("title"))

	var postid string
	if err := row.Scan(&postid); err != nil {
		return err
	}

	for _, id := range tagIDs {
		stmt, err := db.Prepare("INSERT INTO posttags (postid, tagid) VALUES (?, ?)")
		if err != nil {
			return err
		}

		stmt.Exec(postid, id)
	}

	return nil
}
