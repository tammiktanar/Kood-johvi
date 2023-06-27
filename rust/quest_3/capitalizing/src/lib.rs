#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn capitalize_first(input: &str) -> String {
    let mut v: Vec<char> = input.chars().collect();
    if v.len() > 0 {
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        return v.into_iter().collect();

    }
    input.to_string()
}

pub fn title_case(input: &str) -> String {
    let mut res = "".to_string();
    
    for word in input.split_whitespace(){
        res.push(' ');
        let cap_word = capitalize_first(word);

        res.push_str(&cap_word);
    }
    if input.chars().count() == 0{
        return input.to_string()
    } else {

        res.remove(0);

        res
    }
}

pub fn change_case(input: &str) -> String {
    let mut res = "".to_string();

    for cha in input.chars(){
        if !cha.is_whitespace(){
            if cha.is_uppercase(){
                res.push(cha.to_ascii_lowercase());
            } else if cha.is_lowercase(){
                res.push(cha.to_ascii_uppercase());
            } else {
                res.push(cha);
            }
        } else {
            res.push(cha);
        }
    }

    res
}