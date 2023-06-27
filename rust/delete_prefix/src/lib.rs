pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    s.strip_prefix(prefix)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
