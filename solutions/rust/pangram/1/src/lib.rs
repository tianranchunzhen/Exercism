pub fn is_pangram(sentence: &str) -> bool {
    let mut letters: Vec<char> = ('a'..='z').collect();
    for char in sentence.to_ascii_lowercase().chars() {
        if let Some(index) = letters.iter().position(|&c| c == char) {
            letters.remove(index);
        }
    }
    letters.is_empty()
}