#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect::<String>()
}