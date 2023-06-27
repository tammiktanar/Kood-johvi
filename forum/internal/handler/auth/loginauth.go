package auth

import (
	"database/sql"
	"fmt"
	"forum/internal/env"
	"forum/internal/handler/structs"
	"forum/internal/hash"
	"forum/internal/session"
	"net/http"
)

var LoginMsgs structs.LoginMessages

func LoginAuth(env *env.Env) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != "POST" {
			http.Error(w, "400 Bad Request", 400)
			return
		}

		if err := r.ParseForm(); err != nil {
			http.Error(w, "400 Bad Request", 400)
			return
		}

		username := r.FormValue("username")
		password := r.FormValue("password")

		db := env.DB
		if credentialsCorrect(username, password, db, w) { // goes to check if the entered credentials are correct

			row := db.QueryRow("SELECT id FROM users WHERE username = ?", username) // query for getting users Username by ID

			var userid int
			if err := row.Scan(&userid); err != nil {
				http.Error(w, "Something went wrong on our side", 500)
				return
			}

			session.Create(userid, w, r, db) // creates cookie for the user and adds the information to database
			http.Redirect(w, r, "/", 302)
			return

		} else {
			http.Redirect(w, r, "/login", 302)
			return
		}

	}
}

func credentialsCorrect(username string, password string, db *sql.DB, w http.ResponseWriter) bool {
	stmt := fmt.Sprintf("SELECT password FROM users WHERE username = ?")
	row := db.QueryRow(stmt, username)

	var passwordHash string

	switch err := row.Scan(&passwordHash); err {
	case nil:
		if passwordsEq := hash.CheckPasswordHash(password, passwordHash); passwordsEq { // Compare passwords
			return true

		} else {
			LoginMsgs.WrongPassword = true
		}

	case sql.ErrNoRows:
		LoginMsgs.NotFound = true
	default:
		fmt.Println(err)
	}

	return false
}
