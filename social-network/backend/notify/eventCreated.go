package notify

import (
	"fmt"
	"html"
	"social-network/models"
)

type EventCreated struct {
	group   *models.Group
	event   *models.Event
	creator *models.User
	members []*models.User
}

func (n Notifier) EventCreated(
	group *models.Group,
	event *models.Event,
	creator *models.User,
	members []*models.User,
) {
	n.notify(EventCreated{
		group:   group,
		event:   event,
		creator: creator,
		members: members,
	})
}

func (n EventCreated) Targets() []int64 {
	ids := make([]int64, 0, len(n.members)-1)
	for _, member := range n.members {
		if member.UserID != n.creator.UserID {
			ids = append(ids, member.UserID)
		}
	}
	return ids
}

func (n EventCreated) Message() string {
	return fmt.Sprintf(
		"Event <strong>%v</strong> has been created in %v by %v",
		html.EscapeString(n.event.Title),
		html.EscapeString(n.group.Name),
		html.EscapeString(userGetName(n.creator)),
	)
}

func (n EventCreated) Links() []Link {
	return []Link{
		{
			name:   "Show event",
			url:    fmt.Sprintf("/event/%v", n.event.EventID),
			method: "GET",
		},
	}
}
