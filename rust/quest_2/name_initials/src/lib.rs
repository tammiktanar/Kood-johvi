#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/*

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = vec![String::from("lammas"), String::from("lambad")];

    for name in names.iter() {
        let words = name.clone().split(" ");
        let mut res_string = "";

        for word in words {
            let mut letter = word.chars().next().unwrap().to_string().push_str(". ");
            res_string.to_string().push_str(&letter.to_string());
            println!("{}", word);
        }
        
        res_string = res_string.trim_end();
        res.push(res_string.to_string());
        
        println!("{}", "");
    }

    return res
}
*/


pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut answer = vec![];
    let mut first = true;
    for name in names.iter() {
        let mut res = String::from("");
        for char in name.chars() {
            if char.is_uppercase() {
                if !first {
                    res.push_str(" ")
                }
                res.push(char);
                res.push_str(".");
                first = false
            }
        }
        first = true;
        answer.push(res);
    }
    return answer;
}