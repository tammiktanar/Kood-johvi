pub fn number_logic(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    num == num.to_string().chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .map(|n| n.pow(len))
        .reduce(|a, b| a + b)
        .unwrap()
}