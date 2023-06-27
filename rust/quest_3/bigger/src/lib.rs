use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let key_with_max_value = h.iter().max_by_key(|entry | entry.1).unwrap();
    return *key_with_max_value.1
}