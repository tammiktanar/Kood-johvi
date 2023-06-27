package auth

import (
	"forum/internal/env"
	"forum/internal/handler/structs"
	"forum/internal/session"
	"forum/internal/tpl"
	"net/http"
)

type LoginPage struct {
	UserInfo  structs.User
	LoginMsgs structs.LoginMessages
}

func Login(env *env.Env) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		loginPage := LoginPage{
			UserInfo:  session.UserInfo,
			LoginMsgs: LoginMsgs, // created in loginauth.go
		}

		tpl.RenderTemplates(w, "login.html", loginPage, "./templates/auth/login.html", "./templates/base.html")

		LoginMsgs = structs.LoginMessages{} // Reset the login messages or they wont change upon reloading the

	}

}
