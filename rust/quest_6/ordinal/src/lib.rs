#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn num_to_ordinal(nr: u32) -> String {
    match nr {
        11..=19 => nr.to_string() + "th",

        _ => match nr % 10 {
            0 => nr.to_string() + "th",
            1 => nr.to_string() + "st",
            2 => nr.to_string() + "nd",
            3 => nr.to_string() + "rd",
            4..=9 => nr.to_string() + "th",
            _ => unreachable!(),
        }
    }
}