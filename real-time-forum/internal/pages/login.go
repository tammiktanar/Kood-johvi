package pages

import (
	"forum/internal/forumDB"
	"forum/internal/forumEnv"
	"log"
	"net/http"
	"net/url"
	"strings"

	"golang.org/x/crypto/bcrypt"
)

type Login struct {
	forumEnv.Env
}

// Contains things that are generated for every request and passed on to the template
type loginData struct {
	forumEnv.GenericData
	Errors map[string]string // string didn't work for some reason. Maybe we'll add other errors in the future so whatever.
	Inputs url.Values
}

func (env Login) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	data := loginData{}
	data.InitData(env.Env, r)

	if data.User.UserID != 0 { // access denied if logged in
		http.Redirect(w, r, "/board", http.StatusTemporaryRedirect)
		return
	}
	// We must create a new loginData struct because it can't be shared between requests

	data.AddTitle("Login")
	data.Errors = make(map[string]string)

	if r.Method == "POST" {
		r.ParseForm()
		data.Inputs = r.Form

		user, err := env.validate(r)
		if err != nil {
			data.Errors["Error"] = "Incorrect username or password."
		} else {
			env.login(w, r, user)
		}
	}
	// Finally execute the template with the data we got
	tmpl := env.Templates["login"]
	if err := tmpl.ExecuteTemplate(w, "layout", data); err != nil {
		sendErr(err, w, http.StatusInternalServerError)
		return
	}
}

// creates a new session for specified user, only usable in POST request.
func (env Login) login(w http.ResponseWriter, r *http.Request, user *forumDB.User) {
	token, err := env.Sessions.Insert(user.UserID)
	if err != nil {
		log.Panic()
	}

	cookie := &http.Cookie{ // creates new cookie
		Name:   "session",
		Value:  token,
		Path:   "/",   // Otherwise it defaults to /login
		Secure: true,  // true will not work on connections not localhost or HTTPS secured
		MaxAge: 86400, // One day
	}

	w.Header().Add("Set-Cookie", cookie.String())

	log.Printf("%v has logged in.\n", user.Name)
	http.Redirect(w, r, "/board", http.StatusFound)
}

func (env Login) validate(r *http.Request) (*forumDB.User, error) {
	// Get the user information
	user, err := env.Users.GetByName(strings.Title(strings.ToLower(r.FormValue("username"))))
	if err != nil {
		user, err = env.Users.GetByEmail(r.FormValue("username"))
		if err != nil {
			return nil, err
		}
	}

	// Check if the typed in password successfully converts into the matching hash
	if err = bcrypt.CompareHashAndPassword([]byte(user.Password), []byte(r.FormValue("password"))); err != nil {
		return nil, err
	}
	return &user, nil
}
