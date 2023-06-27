#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() > s2.len() {
        for char in s1.chars() {
            if s1.matches(char).count() != s2.matches(char).count() {
                return false
            };
        }

    } else {
        for char in s2.chars() {
            if s1.matches(char).count() != s2.matches(char).count() {
                return false
            };
        }
    }
    
    true
}