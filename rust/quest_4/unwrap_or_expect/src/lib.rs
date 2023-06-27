pub fn odd_to_even(data: Vec<u32>) -> Result<Vec<u32>, (String, Vec<u32>)> {
    let mut a = Vec::new();
    a.extend(data.iter().filter(|&value| value % 2 == 0));
    if a.len() != 0 {
        return Err(("There is a even value in the vector!".to_string(), a));
    }
    a.extend(data.iter().map(|&value| {
        value + 1
    }));
    Ok(a)
}
pub fn expect(v: Vec<u32>) -> Vec<u32> {
    return odd_to_even(v).expect("ERROR ");
}
pub fn unwrap_or(v: Vec<u32>) -> Vec<u32> {
    return odd_to_even(v).unwrap_or(Vec::new());
}
pub fn unwrap_err(v: Vec<u32>) -> (String, Vec<u32>) {
    return odd_to_even(v).unwrap_err();
}
pub fn unwrap(v: Vec<u32>) -> Vec<u32> {
    return odd_to_even(v).unwrap();
}
pub fn unwrap_or_else(v: Vec<u32>) -> Vec<u32> {
    return odd_to_even(v).unwrap_or_else(|err| {err.1});
}