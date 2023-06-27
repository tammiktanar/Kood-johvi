use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, 'b, T, U>(keys: &'a [T], values: &'b [U]) -> HashMap<&'a T, &'b U>
where T: Eq + Hash {
    HashMap::from_iter(keys.iter().zip(values.iter()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
