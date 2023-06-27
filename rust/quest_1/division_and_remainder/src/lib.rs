#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn divide(x: i32, y: i32) -> (i32, i32) {
    return (x / y, x % y)
}