use std::collections::HashMap;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn contain(h: &HashMap<&str, i32>, s: &str) -> bool {
    return h.contains_key(s)
}

pub fn remove(h: &mut HashMap<&str, i32>, s: &str) {
    h.remove_entry(s);
}