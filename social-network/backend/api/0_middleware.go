package api

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"net/http"
	"social-network/router"
	"strconv"
	"time"
)

func ExtendSession(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		cookie, err := r.Cookie("session")
		if err != nil {
			next.ServeHTTP(w, r)
			return
		}

		token := cookie.Value

		success, err := Database.Session.SetExpires(token, sessionDuration)
		if err != nil {
			log.Println(fmt.Errorf("extendSession: %w", err))
			next.ServeHTTP(w, r)
			return
		}

		if success {
			expires := time.Now().Add(sessionDuration)
			newCookie := newSessionCookie(token, expires)
			http.SetCookie(w, newCookie)
		}

		next.ServeHTTP(w, r)
	})
}

func IsAuth(yes http.HandlerFunc, no http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		cookie, err := r.Cookie("session")
		if err != nil {
			no.ServeHTTP(w, r)
			return
		}

		token := cookie.Value
		session, err := Database.Session.Get(token)
		panicUnlessError(err, sql.ErrNoRows)
		if err != nil {
			no.ServeHTTP(w, r)
			return
		}

		ctx := context.WithValue(r.Context(), "session", session)
		yes.ServeHTTP(w, r.WithContext(ctx))
	}
}

// OptionalAuth passes on the request regardless of authorization, but includes session data in the context if it exists.
func OptionalAuth(next http.HandlerFunc) http.HandlerFunc {
	return IsAuth(next, next)
}

func EnsureAuth(next http.HandlerFunc) http.HandlerFunc {
	return IsAuth(
		next,
		func(w http.ResponseWriter, r *http.Request) {
			log.Printf("Request for %v not authorized\n", r.URL.RequestURI())
			writeStatusError(w, http.StatusUnauthorized)
		},
	)
}

func GroupAccessCheck(next http.HandlerFunc) http.HandlerFunc {
	return OptionalAuth(func(w http.ResponseWriter, r *http.Request) {
		groupID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)
		userID := getPossibleUserID(r)

		access, err := Database.Group.HasAccess(groupID, userID)
		if err != nil {
			panic(err)
		}

		if !access {
			log.Printf("Access to group (%v) not authorized\n", groupID)
			writeStatusError(w, http.StatusUnauthorized)
			return
		}

		next.ServeHTTP(w, r)
	})
}

func EventAccessCheck(next http.HandlerFunc) http.HandlerFunc {
	return OptionalAuth(func(w http.ResponseWriter, r *http.Request) {
		eventID, _ := strconv.ParseInt(router.GetSlug(r, 0), 10, 64)
		userID := getPossibleUserID(r)

		access, err := Database.Event.HasAccess(eventID, userID)
		if err != nil {
			panic(err)
		}

		if !access {
			log.Printf("Access to event (%v) not authorized\n", eventID)
			writeStatusError(w, http.StatusUnauthorized)
			return
		}

		next.ServeHTTP(w, r)
	})
}
