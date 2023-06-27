package notify

import (
	"fmt"
	"html"
	"social-network/models"
)

type Invite struct {
	group  *models.Group
	target int64
}

func (n Notifier) Invite(group *models.Group, target int64) {
	n.notify(Invite{
		group:  group,
		target: target,
	})
}

func (n Invite) Targets() []int64 {
	return []int64{n.target}
}

func (n Invite) Message() string {
	return fmt.Sprintf(
		"You have been invited to the group <strong>%v</strong>.",
		html.EscapeString(n.group.Name),
	)
}

func (n Invite) Links() []Link {
	return []Link{
		{
			name: "Join group",
			// url:    fmt.Sprintf("/api/group/%v/join", n.requester),
			url:    fmt.Sprintf("/submit/group/%v/join", n.group.GroupID),
			method: "POST",
		},
	}
}
