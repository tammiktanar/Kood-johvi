package structs

type Post struct {
	ID           int
	Username     string
	Title        string
	Body         string
	CreationDate string
	Tags         []string
	LikeCount    int
	DislikeCount int
}

type Comment struct {
	ID           string
	Body         string
	PostID       int
	UserID       int
	Username     string
	CreationDate string
	Likes        int
	Dislikes     int
}

type User struct {
	ID       int    // ID is for tracking, which user is having a session
	Username string // Display the name of the user who is logged in
	Logged   bool
}

type LoginMessages struct {
	NotFound          bool
	WrongPassword     bool
	SuccesfulRegister bool // Displays message on login screen after succesful registration
	LoginRequired     bool
}

type RegisterMessages struct {
	TakenUn     bool // taken username
	TakenEmail  bool // taken email
	PswrdsNotEq bool // user typed passwords dont match
}
