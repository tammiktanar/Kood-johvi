var submitButtonDiv
var form
var submitButtonDiv
var inputText
var inputSelect

function checkInputText(){
    if (inputText.value.trim() != ""){
        return true;
    } else {
        return false;
    }
}

function borderChange(){
    if (checkInputSelect()){
        inputSelect.style.borderColor = ""
    } else {
        inputSelect.style.borderColor = "red"
    }

    if (checkInputText()){
        inputText.style.borderColor = ""
    } else {
        inputText.style.borderColor = "red"
    }
}

function checkInputSelect(){
    var res = false;
    switch (inputSelect.value) {
        case "Shadow":
            res = true;
            break;
        case "Standard":
            res = true;
            break;
        case "Thinkertoy":
            res = true;
            break;
        default:
            res = false;
            break;
    }

    return res;
}


function initAsciiArtFunc (){
    submitButtonDiv = document.getElementById("submitFormDiv");
    form = document.getElementById("formToSubmit");
    submitButton = document.getElementById("submitForm");
    inputText = document.getElementById("textToConvert");
    inputSelect = document.getElementById("textStyleToConvert");
    
    submitButtonDiv.onclick = function(){
        borderChange();
        if (checkInputSelect() && checkInputText()){
            submitButton.disabled = false;
            form.submit();
        } else {
            submitButton.disabled = true;
        }
    }
    
    submitButtonDiv.onmouseover = function(){
        borderChange();
        if (checkInputSelect() && checkInputText()){
            submitButton.disabled = false;
        } else {
            submitButton.disabled = true;
        }
    }
    
    inputText.onkeyup = function(){
        borderChange();
        if (checkInputSelect() && checkInputText()){
            submitButton.disabled = false;
        } else {
            submitButton.disabled = true;
        }
    };
    
    
    inputSelect.onchange = function(){
        borderChange();
        if (checkInputSelect() && checkInputText()){
            submitButton.disabled = false;
        } else {
            submitButton.disabled = true;
        }
    }
  
    borderChange();
}
