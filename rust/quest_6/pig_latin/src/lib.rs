pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";
    text.split_whitespace()
    .map(|word| {
        let mut new = word.trim_start_matches(|ch| !vowels.contains(ch));
        let mut removed = word.len() - new.len();
        if &word[removed.saturating_sub(1)..=removed] == "qu" {
            removed += 1;
            new = &word[removed..];
        }
        let consonants = &word[0..removed];
        String::with_capacity(word.len() + 2) + new + consonants + "ay"
    })
    .collect()
}