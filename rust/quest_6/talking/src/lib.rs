pub fn talking(text: &str) -> &str {
    let mut question = false;   
    let mut letters = false;
    for char in text.chars() {
        if char.is_alphabetic(){
            letters = true;
        }
    }
    if text.trim().is_empty() {
        return "Just say something!";
    } 
    if text.trim() == "?" {
        return "Sure.";
    }
    let last = text.to_string().chars().last().unwrap();
    print!("{}", text);
    if last == '?' {
        question = true;
        }
    if letters && text.to_uppercase() == text {
        if question {
            return "Quiet, I am thinking!";
        }
        return "There is no need to yell, calm down!";
    }
    if question {
        return "Sure.";
    }
    return "Interesting";
}