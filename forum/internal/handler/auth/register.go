package auth

import (
	"forum/internal/handler/structs"
	"forum/internal/session"
	"forum/internal/tpl"
	"net/http"
)

type RegisterPage struct {
	RegMsgs  structs.RegisterMessages
	UserInfo structs.User
}

func Register() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		registerPage := RegisterPage{
			RegMsgs:  RegMsgs,          // RegMsgs is created in registerauth.go
			UserInfo: session.UserInfo, // UserInfo is in session/check.go
		}

		tpl.RenderTemplates(w, "register.html", registerPage, "./templates/auth/register.html", "./templates/base.html")

		RegMsgs = structs.RegisterMessages{} // Reset the login messages or they wont change upon reloading the page
	}

}
