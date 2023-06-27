#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}