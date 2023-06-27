package api

import (
	"database/sql"
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"social-network/models"
	"social-network/router"
	"strconv"
)

func GetUserBySession(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	user, err := Database.User.GetByID(session.UserID)
	panicUnlessError(err, sql.ErrNoRows)
	if err != nil {
		http.NotFound(w, r)
		return
	}

	writeJSON(w, user)
}

func GetUserByID(w http.ResponseWriter, r *http.Request) {
	// GET /api/user/([0-9]+)
	myID := getPossibleUserID(r)
	slug := router.GetSlug(r, 0)

	id, _ := strconv.ParseInt(slug, 10, 64)

	user, err := Database.User.GetByIDPlusFollowInfo(id, myID)
	panicUnlessError(err, sql.ErrNoRows)
	if err != nil {
		http.NotFound(w, r)
		return
	}

	if myID != id && (user.Private && !user.FollowInfo.MeToYou) {
		payload := struct {
			*models.UserLimited
			Access bool `json:"access"`
		}{
			UserLimited: user.Limited(),
			Access:      false,
		}

		writeJSON(w, payload)
		return
	}

	payload := struct {
		*models.User
		Access bool `json:"access"`
	}{
		User:   user,
		Access: true,
	}

	writeJSON(w, payload)
}

func GetUserByEmail(w http.ResponseWriter, r *http.Request) {
	// GET /api/user/([^/]+)
	email := router.GetSlug(r, 0)

	user, err := Database.User.GetByEmail(email)

	panicUnlessError(err, sql.ErrNoRows)
	if err != nil {
		writeStatusError(w, http.StatusNotFound)
		return
	}

	writeJSON(w, user)
}

func UpdateUser(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	user := models.UserIncoming{}

	err := json.NewDecoder(r.Body).Decode(&user)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	err = Database.User.Update(session.UserID, user)
	log.Printf("%T\n", errors.Unwrap(err))
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}
}

func UserFollow(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	slug := router.GetSlug(r, 0)
	targetID, _ := strconv.ParseInt(slug, 10, 64)

	target, err := Database.User.GetByID(targetID)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	if target.Private {
		// Sending follow request to private user
		err = Database.User.RequestFollow(session.UserID, target.UserID)
		if err != nil {
			panic(err)
		}

		go func() {
			me, err := Database.User.GetByID(session.UserID)
			if err != nil {
				log.Println(err)
			}

			Notify.FollowRequest(me, targetID)
		}()
	} else {
		// Following a public user
		err = Database.User.Follow(session.UserID, target.UserID)
		if err != nil {
			panic(err)
		}

		go func() {
			me, err := Database.User.GetByID(session.UserID)
			if err != nil {
				log.Println(err)
			}
			Notify.Follow(me, targetID)
		}()
	}
}

func UserAcceptFollow(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	slug := router.GetSlug(r, 0)
	targetID, _ := strconv.ParseInt(slug, 10, 64)

	err := Database.User.FollowAccept(session.UserID, targetID)
	if err != nil {
		panic(err)
	}

	go func() {
		me, err := Database.User.GetByID(session.UserID)
		if err != nil {
			log.Println(err)
		}

		Notify.FollowAccepted(me, targetID)
	}()
}

func UserUnfollow(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	slug := router.GetSlug(r, 0)
	targetID, _ := strconv.ParseInt(slug, 10, 64)

	err := Database.User.Unfollow(session.UserID, targetID)
	if err != nil {
		panic(err)
	}
}

func UserFollowers(w http.ResponseWriter, r *http.Request) {
	slug := router.GetSlug(r, 0)
	targetID, _ := strconv.ParseInt(slug, 10, 64)

	users, err := Database.User.ListFollowers(targetID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, users)
}

func UserFollowing(w http.ResponseWriter, r *http.Request) {
	slug := router.GetSlug(r, 0)
	targetID, _ := strconv.ParseInt(slug, 10, 64)

	users, err := Database.User.ListFollowing(targetID)
	if err != nil {
		panic(err)
	}

	writeJSON(w, users)
}

func GetKnownUsers(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	users, err := Database.User.Known(session.UserID)
	panicIfErr(err)

	writeJSON(w, users)
}
