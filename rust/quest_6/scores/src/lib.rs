#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn score(sentence: &str) -> u64 {
    let mut res:u64 = 0;

    for cha in sentence.chars() {
        res += match cha.to_ascii_uppercase().to_string().as_str() {
            "A" | "E" | "I" | "O" | "U" | "L" | "N" | "R" | "S" | "T" => 1,
            "D" | "G" => 2,
            "B" | "C" | "M" | "P" => 3,
            "F" | "H" | "V" | "W" | "Y" => 4,
            "K" => 5,
            "J" | "X" => 8,
            "Q" | "Z" => 10,
            _ => 0
        }
    }


    return res
}