#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arrange_phrase_test() {
        assert_eq!("This is a Test", arrange_phrase("is2 Thi1s T4est 3a"))
    }
}



pub fn arrange_phrase(phrase: &str) -> String {
    let mut first = true;
    let strings: Vec<&str> = phrase.split(" ").collect();
    let mut answer = String::new();
    let len = strings.len();
    for i in 0..len+1 {
        
        for word in &strings {

            if word.contains(char::from_digit(i as u32, 10).unwrap()) {

                if !first{
                    answer.push_str(" ");
                }
                first = false;
                answer.push_str(&word.replace(char::from_digit(i as u32, 10).unwrap(), ""));
            }
        }
    }
    
    return answer;
}