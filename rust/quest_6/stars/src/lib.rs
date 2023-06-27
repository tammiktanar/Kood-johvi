#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn stars(n: u32) -> String {
    let res = "*".repeat(2_usize.pow(n));
    return res
}