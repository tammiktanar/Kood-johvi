package auth

import (
	"database/sql"
	"forum/internal/env"
	"forum/internal/handler/structs"
	"forum/internal/hash"
	"log"
	"net/http"
)

var RegMsgs structs.RegisterMessages

func RegisterAuth(env *env.Env) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != "POST" {
			http.Error(w, "Bad request!", 400)
			return
		}

		if err := r.ParseForm(); err != nil {
			http.Error(w, "Bad request!", 400)
			return
		}

		var (
			username  = r.FormValue("username")
			email     = r.FormValue("email")
			password1 = r.FormValue("password")
			password2 = r.FormValue("password2")
		)

		db := env.DB
		invalidInput := false // Checks if the credentials that user wrote are valid

		if rowExists("SELECT username FROM users WHERE username = ?", username, db) { // if the username exists
			RegMsgs.TakenUn = true
			invalidInput = true
		}

		if rowExists("SELECT email from USERS WHERE email = ?", email, db) { // if the email exists
			RegMsgs.TakenEmail = true
			invalidInput = true
		}

		if !(password1 == password2) { // if the passwords arent same
			RegMsgs.PswrdsNotEq = true
			invalidInput = true
		}

		if !invalidInput {

			password1, _ := hash.Password(password1)
			addUser(username, password1, email, db) // add user to the database

			LoginMsgs.SuccesfulRegister = true // This is to tell user on login page whether their registration was succesful
			http.Redirect(w, r, "/login", 302)
			return

		} else {
			http.Redirect(w, r, "/register", 302)
			return
		}
	}
}

func addUser(username, password, email string, db *sql.DB) {
	stmt, err := db.Prepare("INSERT INTO users (username, password, email) VALUES (?, ?, ?)")
	if err != nil {
		log.Fatal(err)
	}
	stmt.Exec(username, password, email)
}

func rowExists(q string, value string, db *sql.DB) bool {
	row := db.QueryRow(q, value)

	switch err := row.Scan(&value); err {

	case sql.ErrNoRows:
		return false

	case nil:
		return true

	default:

		return false
	}
}
