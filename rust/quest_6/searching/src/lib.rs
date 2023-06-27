#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().position(|&index| index == key)
}
