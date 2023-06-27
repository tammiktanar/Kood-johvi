package notify

import (
	"fmt"
	"html"
	"social-network/models"
)

type FollowAccepted struct {
	accepter *models.User
	target   int64
}

func (n Notifier) FollowAccepted(accepter *models.User, target int64) {
	n.notify(FollowAccepted{
		accepter: accepter,
		target:   target,
	})
}

func (f FollowAccepted) Targets() []int64 {
	return []int64{f.target}
}

func (f FollowAccepted) Message() string {
	return fmt.Sprintf("You are now following <strong>%v</strong>!", html.EscapeString(userGetName(f.accepter)))
}

func (f FollowAccepted) Links() []Link {
	return []Link{
		{
			name:   "See their profile",
			url:    fmt.Sprintf("/user/%v", f.accepter.UserID),
			method: "GET",
		},
	}
}
