package api

import (
	"encoding/json"
	"log"
	"net/http"
	"social-network/models"
	"social-network/router"
	"strconv"
	"time"
)

func CreateComment(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)
	postID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)

	comment := models.Comment{}

	err := json.NewDecoder(r.Body).Decode(&comment)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	// Check if you have access to the post
	access, err := Database.Post.HasAccess(session.UserID, postID)
	panicIfErr(err)
	if !access {
		log.Println("Don't have access to post")
		writeStatusError(w, http.StatusForbidden)
		return
	}

	comment.PostID = postID
	comment.AuthorID = session.UserID

	id, err := Database.Comment.Insert(comment)
	panicIfErr(err)

	comment.CommentID = id
	comment.Created = time.Now()

	writeJSON(w, comment)
}

func GetCommentsByPost(w http.ResponseWriter, r *http.Request) {
	myID := getPossibleUserID(r)
	postID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)

	// Check if you have access to the post
	access, err := Database.Post.HasAccess(myID, postID)
	panicIfErr(err)
	if !access {
		log.Println("Don't have access to post")
		writeStatusError(w, http.StatusForbidden)
		return
	}

	comments, err := Database.Comment.GetByPost(postID)
	panicIfErr(err)

	writeJSON(w, comments)
}
