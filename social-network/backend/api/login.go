package api

import (
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"social-network/models"
	"time"
)

// Login handler reads in credentials and sends back a session token.
func Login(w http.ResponseWriter, r *http.Request) {
	// Decode credentials
	credentials := struct {
		Email    string `json:"email"`
		Password string `json:"password"`
	}{}

	err := json.NewDecoder(r.Body).Decode(&credentials)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	// Check if the user exists
	user, err := Database.User.GetByEmail(credentials.Email)
	panicUnlessError(err, sql.ErrNoRows)
	if err != nil {
		writeStatusError(w, http.StatusUnauthorized)
		return
	}

	// Check password
	// TODO: Password encryption
	if user.Password != credentials.Password {
		writeStatusError(w, http.StatusUnauthorized)
		return
	}

	// Valid credentials at this point
	doLogin(w, user)
	writeJSON(w, user)
}

func doLogin(w http.ResponseWriter, user *models.User) {
	token, err := Database.Session.Insert(user.UserID, sessionDuration)
	if err != nil {
		panic(err)
	}

	cookie := newSessionCookie(token, time.Now().Add(sessionDuration))
	http.SetCookie(w, cookie)
}

// Logout handler takes in session token and removes it.
func Logout(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	token := session.Token

	success, err := Database.Session.Delete(token)
	if err != nil {
		panic(err)
	}

	if !success {
		writeStatusError(w, http.StatusUnauthorized)
		return
	}

	cookie := newSessionCookie("deleted", time.Unix(0, 0))
	http.SetCookie(w, cookie)

	w.WriteHeader(http.StatusNoContent)
}

// LogoutAll handler logs a user out from all their sessions.
func LogoutAll(w http.ResponseWriter, r *http.Request) {
	session := getSession(r)

	user, err := Database.User.GetByID(session.UserID)
	if err != nil {
		panic(err)
	}

	err = Database.Session.ClearUser(user.UserID)
	if err != nil {
		panic(err)
	}

	cookie := newSessionCookie("deleted", time.Unix(0, 0))
	http.SetCookie(w, cookie)

	w.WriteHeader(http.StatusNoContent)
}

func Register(w http.ResponseWriter, r *http.Request) {
	// POST /api/register

	// Create custom struct because the user struct doesn't include json tag for password
	incoming := models.UserIncoming{}

	err := json.NewDecoder(r.Body).Decode(&incoming)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	id, err := Database.User.Insert(incoming)
	if err != nil {
		log.Println(err)
		writeStatusError(w, http.StatusBadRequest)
		return
	}

	user, err := Database.User.GetByID(id)
	if err != nil {
		panic(err)
	}

	doLogin(w, user)
	writeJSON(w, user)
}

// func registerValidate(user models.User) *map[string]string {
// 	return nil
// }
