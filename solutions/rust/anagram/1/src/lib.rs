use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_letters: Vec<char> = word_lower.chars().collect();
    word_letters.sort();

    possible_anagrams
        .iter()
        .filter(|&&anagram| {
            let anagram_lower = anagram.to_lowercase();
            anagram_lower != word_lower && {
                let mut anagram_letters = anagram_lower.chars().collect::<Vec<char>>();
                anagram_letters.sort();
                anagram_letters == word_letters
            }
        })
        .copied()
        .collect()
}
