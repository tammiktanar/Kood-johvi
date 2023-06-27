package main

import (
	"log"
	"net/http"
	"os"
	"social-network/api"
	"social-network/database"
	"social-network/notify"
	"social-network/router"
	"time"
)

func main() {
	_ = os.Mkdir("./persist", os.ModePerm)
	db := database.NewDatabase("./persist/database.db")
	api.Database = db
	api.Notify = notify.NewNotifier(db)

	go CleanupDB(db)

	rtr := router.New()

	rtr.Get("/api/user", api.EnsureAuth(api.GetUserBySession))
	rtr.Post("/api/user", api.EnsureAuth(api.UpdateUser))
	rtr.Get("/api/user/([0-9]+)", api.OptionalAuth(api.GetUserByID))
	rtr.Get("/api/user/known", api.EnsureAuth(api.GetKnownUsers))
	rtr.Get("/api/user/([^/]+)", api.GetUserByEmail)

	rtr.Get("/api/user/([0-9]+)/followers", api.UserFollowers)
	rtr.Get("/api/user/([0-9]+)/following", api.UserFollowing)
	rtr.Post("/api/user/([0-9]+)/follow", api.EnsureAuth(api.UserFollow))
	rtr.Post("/api/user/([0-9]+)/accept", api.EnsureAuth(api.UserAcceptFollow))
	rtr.Post("/api/user/([0-9]+)/unfollow", api.EnsureAuth(api.UserUnfollow))

	rtr.Post("/api/register", api.Register)
	rtr.Post("/api/login", api.Login)
	rtr.Get("/api/logout", api.EnsureAuth(api.Logout))
	rtr.Get("/api/logout/all", api.EnsureAuth(api.LogoutAll))

	rtr.Post("/api/post/create", api.EnsureAuth(api.CreatePost))
	rtr.Get("/api/post/([0-9]+)", api.OptionalAuth(api.GetPostByID))
	rtr.Get("/api/post/all", api.OptionalAuth(api.GetAllPosts))
	rtr.Get("/api/post/all/groups", api.EnsureAuth(api.GetMyGroupPosts))
	rtr.Get("/api/post/all/following", api.EnsureAuth(api.GetMyFollowingPosts))
	rtr.Get("/api/group/([0-9]+)/posts", api.GroupAccessCheck(api.GetGroupPosts))
	rtr.Get("/api/user/([0-9]+)/posts", api.OptionalAuth(api.GetUserPosts))

	rtr.Post("/api/post/([0-9]+)/comment/create", api.EnsureAuth(api.CreateComment))
	rtr.Get("/api/post/([0-9]+)/comment/all", api.OptionalAuth(api.GetCommentsByPost))

	rtr.Get("/api/file/([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})", api.FileDownload)
	rtr.Post("/api/file", api.FileUpload)

	rtr.Post("/api/group/create", api.EnsureAuth(api.CreateGroup))
	rtr.Get("/api/group/all", api.OptionalAuth(api.GetAllGroups))
	rtr.Get("/api/group/my", api.OptionalAuth(api.GetMyGroups))
	rtr.Get("/api/group/([0-9]+)", api.GroupAccessCheck(api.GetGroupByID))
	rtr.Post("/api/group/([0-9]+)/invite/([0-9]+)", api.EnsureAuth(api.GroupInvite))
	rtr.Post("/api/group/([0-9]+)/join", api.EnsureAuth(api.JoinGroup))
	rtr.Post("/api/group/([0-9]+)/leave", api.EnsureAuth(api.LeaveGroup))
	rtr.Get("/api/group/([0-9]+)/members", api.GroupAccessCheck(api.GetGroupMembers))
	rtr.Post("/api/group/([0-9]+)/transfer/([0-9]+)", api.EnsureAuth(api.TransferOwnership))
	rtr.Get("/api/group/([0-9]+)/invite/all", api.GroupAccessCheck(api.GetPendingInvites))

	rtr.Post("/api/event/create", api.EnsureAuth(api.CreateEvent))
	rtr.Post("/api/event/([0-9]+)/going", api.EnsureAuth(api.EventGoing))
	rtr.Post("/api/event/([0-9]+)/not-going", api.EnsureAuth(api.EventNotGoing))
	rtr.Post("/api/event/([0-9]+)/unset", api.EnsureAuth(api.EventUnset))
	rtr.Get("/api/event/([0-9]+)", api.EventAccessCheck(api.GetEvent))
	rtr.Get("/api/group/([0-9]+)/events", api.GroupAccessCheck(api.GetGroupEvents))
	rtr.Get("/api/event/([0-9]+)/members", api.EventAccessCheck(api.GetEventMembers))
	rtr.Get("/api/event/all", api.EnsureAuth(api.GetMyEvents))

	// rtr.Get("/api/chat", api.EnsureAuth(api.StartChatWebsocket))

	rtr.Post("/api/message/send", api.EnsureAuth(api.SendMessage))
	rtr.Post("/api/message/history", api.EnsureAuth(api.GetMessages))

	// wsRouter := http.NewServeMux()
	// wsRouter.HandleFunc("/", api.StartChatWebsocket)

	// TODO: Allow setting port with command line flag.
	log.Println("Backend listening on http://localhost:8888")
	log.Panic(http.ListenAndServe(":8888", router.ApplyMiddleware(
		rtr,
		api.ExtendSession,
		router.RedirectTrailingSlash,
		router.LogRequests,
		router.Recover500,
	)),
	)
}

func CleanupDB(db *database.Database) {
	for {
		n, err := db.Session.CleanExpired()
		if err != nil {
			log.Printf("Failed to clean up old sessions: %v\n", err)
		}
		log.Printf("Cleaned up %v old sessions\n", n)
		time.Sleep(time.Hour * 24)
	}
}
