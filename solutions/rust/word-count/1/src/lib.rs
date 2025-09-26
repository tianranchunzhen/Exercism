use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut res = HashMap::new();
    words
        .to_ascii_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .map(|word| word.trim_matches('\'').to_string())
        .filter(|word| !word.is_empty())
        .for_each(|word| *res.entry(word).or_insert(0) += 1);
    res
}
