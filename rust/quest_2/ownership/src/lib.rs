#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn first_subword(s: String) -> String {
    let mut first = true;
    let mut res = "".to_string();

    for character in s.chars(){
        if !first {
            if character == '_' || character.is_uppercase() {
                break
            }
        }

        res.push(character);
        first = false;
    } 

    return res
}