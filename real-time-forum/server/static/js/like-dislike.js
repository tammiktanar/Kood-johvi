function likeComment(element) {
    let postID = $(element).attr("postid");
    $.ajax({
        url: "/like?id=" + postID,
        method: "post",
        dataType: "json", // We are getting back json from the server
        success: (response) => {
            if (response.RedirectPath) {
                $(location).attr("href", response.RedirectPath)
                return;
            }

            $("#likes-" + postID).text(response.LikeCount);
            $(element).toggleClass("like-dislike-highlight");
            $(".shown-dislike-button[postid='" + postID + "']").removeClass("like-dislike-highlight")
        }
    });
}
function dislikeComment(element){
    let postID = $(element).attr("postid");
    $.ajax({
        url: "/dislike?id=" + postID,
        method: "post",
        dataType: "json", // We are getting back json from the server
        success: (response) => {
            if (response.RedirectPath) {
                $(location).attr("href", response.RedirectPath)
                return;
            }

            $("#likes-" + postID).text(response.LikeCount);
            $(element).toggleClass("like-dislike-highlight");
            $(".shown-like-button[postid='" + postID + "']").removeClass("like-dislike-highlight")
        }
    });
}