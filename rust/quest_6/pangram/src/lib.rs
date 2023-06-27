#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn is_pangram(s: &str) -> bool {
    if s.is_empty() {
        return false
    }

    let mut alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    for cha in s.to_ascii_lowercase().chars() {
        if alphabet.len() != 0 {
            let index = alphabet.iter().position(|x| *x == cha);
            if index >= Some(0) {
                alphabet.remove(index.unwrap());
            }
        } else {
            break
        }
    }



    return alphabet.len() == 0
}