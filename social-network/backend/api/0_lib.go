package api

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"social-network/database"
	"social-network/models"
	"social-network/notify"
	"strconv"
	"time"
)

var Database *database.Database
var Notify *notify.Notifier

const sessionDuration = 24 * time.Hour

// writeJSON sends a response in JSON format. Panics if an error occurs.
func writeJSON(w http.ResponseWriter, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	err := json.NewEncoder(w).Encode(payload)
	if err != nil {
		panic(err)
	}
}

// panicUnlessError checks if there's an error, and panics if it's not nil, unless err is of type unless
func panicUnlessError(err error, unless ...error) {
	if err == nil || _checkAnyErrIs(err, unless) {
		return
	}
	panic(err)
}

func _checkAnyErrIs(err error, checks []error) bool {
	for _, check := range checks {
		if errors.Is(err, check) {
			return true
		}
	}
	return false
}

// writeStatusError writes a response according to the status code.
func writeStatusError(w http.ResponseWriter, code int) {
	http.Error(w, fmt.Sprintf(`%v %v`, code, http.StatusText(code)), code)
}

// newSessionCookie makes a new cookie for a given session token
func newSessionCookie(token string, expires time.Time) *http.Cookie {
	return &http.Cookie{
		Name:     "session",
		Value:    token,
		Path:     "/",
		Expires:  expires,
		HttpOnly: true,
	}
}

func getSession(r *http.Request) *models.Session {
	session, ok := r.Context().Value("session").(*models.Session)
	if !ok {
		panic("Can't get session from a handler that hasn't been authenticated. Use api.IsAuth and api.EnsureAuth.")
	}

	return session
}

// Returns the session's userID. If session is invalid, returns -1
func getPossibleUserID(r *http.Request) int64 {
	var userID int64 = -1
	session, ok := r.Context().Value("session").(*models.Session)
	if ok {
		userID = session.UserID
	}

	return userID
}

// Panics if err != nil
func panicIfErr(err error) {
	if err != nil {
		panic(err)
	}
}

// Turns a query string into an int. If string is empty, returns -1
func queryAtoi(s string) (int64, error) {
	if s == "" {
		return -1, nil
	}

	id, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		return -1, fmt.Errorf("bad int: %v", s)
	}

	return id, nil
}
