package api

import (
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"social-network/models"
	"social-network/router"
	"strconv"
	"strings"
	"time"
)

func CreatePost(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	post := struct {
		models.Post
		AllowedUsers []int64 `json:"allowedUsers"`
	}{}

	err := json.NewDecoder(r.Body).Decode(&post)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	// If it's a group post, check that I have access
	if post.GroupID != nil {
		post.Privacy = "public"

		access, err := Database.Group.IncludesUser(*post.GroupID, session.UserID)
		panicIfErr(err)
		if !access {
			writeStatusError(w, http.StatusForbidden)
			return
		}
	}

	if post.Privacy == "manual" {
		if post.AllowedUsers == nil {
			log.Println("Tried to insert a post with privacy \"MANUAL\", but with no allowedUsers array defined")
			writeStatusError(w, http.StatusBadRequest)
			return
		}

		// This is a crappy way of doing this, but I want to make sure the provided user IDs are valid
		// That way we don't end up with an "orphan" post that nobody has access to
		// A much better alternative would be to use a transaction, but we don't have the framework set up for that
		for i, userID := range post.AllowedUsers {
			_, err := Database.User.GetByID(userID)
			if err != nil {
				log.Printf("Provided userID %v at allowedUsers[%v] is not valid\n", userID, i)
				writeStatusError(w, http.StatusBadRequest)
				return
			}
		}
	}

	for _, img := range strings.Split(post.Images, ",") {
		if img == "" {
			continue
		}

		_, err = Database.File.Get(img)
		if err != nil {
			log.Printf("Could not find file with token %v\n", img)
			writeStatusError(w, http.StatusBadRequest)
			return
		}
	}

	post.AuthorID = session.UserID
	id, err := Database.Post.Insert(post.Post)
	if err != nil {
		panic(err)
	}

	if post.Privacy == "manual" {
		for _, userID := range post.AllowedUsers {
			err = Database.Post.InsertAllowedUser(id, userID)
			panicIfErr(err)
		}
	}

	post.PostID = id
	post.Created = time.Now()

	writeJSON(w, post.Post)
}

func GetAllPosts(w http.ResponseWriter, r *http.Request) {
	myID := getPossibleUserID(r)

	beforeID, err := queryAtoi(r.URL.Query().Get("beforeID"))
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	posts, err := Database.Post.GetAll(myID, beforeID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, posts)
}

func GetPostByID(w http.ResponseWriter, r *http.Request) {
	myID := getPossibleUserID(r)
	slug := router.GetSlug(r, 0)
	postID, _ := strconv.ParseInt(slug, 10, 64)

	allowed, err := Database.Post.HasAccess(myID, postID)
	panicUnlessError(err, sql.ErrNoRows)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusNotFound)
		return
	}
	if !allowed {
		writeStatusError(w, http.StatusForbidden)
		return
	}

	post, err := Database.Post.GetByID(postID)
	panicIfErr(err)

	writeJSON(w, post)
}

func GetGroupPosts(w http.ResponseWriter, r *http.Request) {
	groupID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)

	beforeID, err := queryAtoi(r.URL.Query().Get("beforeID"))
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	posts, err := Database.Post.GetByGroup(groupID, beforeID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, posts)
}

func GetMyGroupPosts(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	beforeID, err := queryAtoi(r.URL.Query().Get("beforeID"))
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	posts, err := Database.Post.GetByMyGroups(session.UserID, beforeID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, posts)
}

func GetUserPosts(w http.ResponseWriter, r *http.Request) {
	myID := getPossibleUserID(r)

	slug := router.GetSlug(r, 0)
	userID, _ := strconv.ParseInt(slug, 10, 64)

	beforeID, err := queryAtoi(r.URL.Query().Get("beforeID"))
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	posts, err := Database.Post.GetByUser(myID, userID, beforeID)
	panicIfErr(err)

	writeJSON(w, posts)
}

func GetMyFollowingPosts(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	beforeID, err := queryAtoi(r.URL.Query().Get("beforeID"))
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	posts, err := Database.Post.GetByFollowing(session.UserID, beforeID)
	panicIfErr(err)

	writeJSON(w, posts)
}
