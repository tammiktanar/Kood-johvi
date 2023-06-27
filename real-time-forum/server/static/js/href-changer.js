import * as Ws from "/js/modules/websocket.js"

document.onmouseover = function() {
    window.innerDocClick = true;
}

document.onmouseleave = function() {
    window.innerDocClick = false;
}


window.onpopstate = function() {
    if (!window.innerDocClick) {
        loadContentFromLink(window.location.href, true, false)
    }
}

window.addEventListener('load', () => {
    document.getElementById("search-form").onsubmit =function (e) {
        var element = e.target 
        
        searchForm(element)

        return false
    };
});
  

document.onclick = function (e) {
    e = e ||  window.event;
    var element = e.target || e.srcElement;
  
    if (element.tagName == 'A') {
        const {link, changeContent} = checkURLSpecialCase(element.href)

        loadContentFromLink(link, changeContent);
        return false; // prevent default action and stop event propagation
    } else if (element.tagName == 'INPUT'){
        let form = element.parentElement
        let className = element.parentElement.className
        switch (className) {
            case "login-form": // Login override
                loginForm(form)

                return false;
            case "register-form": // Register override
                registerForm(form)
                
                return false
            default:
                if (form.parentElement.id == "new-thread-form" && element.id == "new-thread-submit"){ // Thread creation override
                    newThreadForm(form.parentElement)

                    return false
                } else {
                    let textAreas = form.getElementsByTagName("textarea")
                    if (textAreas){
                        if (textAreas["new-post-textarea"]){
                            newPostForm(form)

                            return false
                        }
                    }

                }
                break;
        }
    } else if (element.tagName == 'DIV'){
        if (element.id =="settings-href"){
            loadContentFromLink("/settings")
        } else if (element.id =="save-settings"){
            saveSettings()
        }
    } else if (element.tagName.toUpperCase() == 'SVG'){
        if (element.classList.contains('shown-dislike-button')){
            dislikeComment(element)
        } else if (element.classList.contains('shown-like-button')) {
            likeComment(element)
        }
    } else if (element.tagName.toUpperCase() == 'PATH'){
        if (element.parentElement){
            if (element.parentElement.classList.contains('shown-dislike-button')){
                dislikeComment(element.parentElement)
            } else if (element.parentElement.classList.contains('shown-like-button')) {
                likeComment(element.parentElement)
    
            }
        }
    }
};


function loadContentFromContent(content){
    if (content) document.documentElement.innerHTML = content;
    Ws.loadChatIntoBody()

    document.getElementById("search-form").onsubmit =function (e) {
        var element = e.target 
        
        searchForm(element)

        return false
    };
}

function loadContentFromLink(link, changeContent = true, changeHistory = true){
    $.ajax({
        type:"GET",
        url: link,
        crossDomain:false,
            beforeSend: function(xhr) 
        {
            xhr.overrideMimeType('text/plain; charset=UTF-8');
        },
        success:function(data) {
            if (changeContent) document.documentElement.innerHTML = data;
            if (changeHistory) window.history.pushState("", "", link);
            Ws.loadChatIntoBody()

            document.getElementById("search-form").onsubmit =function (e) {
                var element = e.target 
                
                searchForm(element)
        
                return false
            };
        },
        error:function(errorStatus,xhr) {
            alert("Error",errorStatus,xhr);
        }
    });
}



function checkURLSpecialCase(link){
    const changeContent = true

    return {link: link, changeContent: changeContent}
}



function loginForm(form){
    let inputs = form.getElementsByTagName("input")

    $.ajax({
        type: "POST",
        url: "/login",
        data: {
            username: inputs.username.value,
            password: inputs.password.value,
        },
        success: function(data)
        {
            loadContentFromContent(data);
            Ws.openWebsocket()
            Ws.loadChatIntoBody()
        }
    });

    return 
}

function registerForm(form){
    let inputs = form.getElementsByTagName("input")
    
    $.ajax({
        type: "POST",
        url: "/register",
        data: {
            "username": inputs.username.value,
            "password": inputs.password.value,
            "confirm-password": inputs["confirm-password"].value,
            "email": inputs["email"].value,
            "confirm-email": inputs["confirm-email"].value,
            "birthday": inputs["birthday"].value,
            "gender": inputs["gender"].value,
            "first-name": inputs["first-name"].value,
            "last-name": inputs["last-name"].value,
        },
        success: function(data)
        {
            loadContentFromContent(data);
        }
    });
    
    return 
}

function newThreadForm(form){
    let inputs = form.getElementsByTagName("input")
    let textAreas = form.getElementsByTagName("textarea")

    $.ajax({
        type: "POST",
        url: window.location.href,
        data: {
            "title": inputs["title"].value,
            "content": textAreas["content"].value,
            "tags": inputs["tags"].value,

        },
        success: function(data)
        {
            loadContentFromContent(data);
            let postPage = document.getElementById("posts-page")
            let link = (postPage.children[0].href.split("&")[0])
            window.history.pushState("", "", (link))
        }
    });

    return 
}

function newPostForm(form){
    let textAreas = form.getElementsByTagName("textarea")
    textAreas["post"]

    $.ajax({
        type: "POST",
        url: window.location.href,
        data: {
            "post": textAreas["post"].value,


        },
        success: function(data)
        {
            loadContentFromContent(data);
        },
        error: function()
        {
            loadContentFromLink(window.location.href);
        }
    });

    return
}


function searchForm(form){
    let inputs = form.getElementsByTagName("input")

    $.ajax({
        type: "POST",
        url: "/search",
        data: {
            "search": inputs["search"].value,
        },
        success: function(data)
        {  
            window.history.pushState("", "", (this.url + "?" + this.data))
            loadContentFromContent(data);
        },
    });


    return false
}