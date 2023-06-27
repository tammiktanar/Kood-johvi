#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub validation: bool,
    pub expected: String,
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        Self {validation: validation, expected: expected}
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let alph: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let reversed: Vec<_> = "zyxwvutsrqponmlkjihgfedcbaZYXWVUTSRQPONMLKJIHGFEDCBA".chars().collect();
    let mut ciphered_string = "".to_string();

    if original.is_empty() || ciphered.is_empty() {
        return None
    }

    for cha in original.chars(){
        if alph.contains(&cha) {
            let index = alph.iter()
            .position(|&x| x == cha)
            .unwrap();
            ciphered_string.push(*reversed.get(index)?)
        } else {
            ciphered_string.push(cha)
        }
    }

    if ciphered_string == ciphered {
        return Some(Ok(true));
    }

    Some(Err(CipherError{validation: false, expected: ciphered_string}))
}