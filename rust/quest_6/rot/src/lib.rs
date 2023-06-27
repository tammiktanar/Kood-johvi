#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn rotate(input: &str, key: i8) -> String {
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut res = String::from("");

    for cha in input.chars() {
        if alpha.contains(cha) {
            let mut rot = alpha.chars().position(|c| c == cha).unwrap() as i8;
            if rot+key > 25 {
                rot -= 26;
            }
            if rot + key < 0 {
                rot += 26;
            }

            res.push(alpha.chars().nth((rot + key) as usize).unwrap());
        }
        else if upper.contains(cha) {
            let mut rot = upper.chars().position(|c| c == cha).unwrap() as i8;
            if rot+key > 25 {
                rot -= 26;
            }
            if rot + key < 0 {
                rot += 26;
            }

            res.push(upper.chars().nth((rot + key) as usize).unwrap());
        } else {
            res.push(cha);
        }
    }

    return res;
}