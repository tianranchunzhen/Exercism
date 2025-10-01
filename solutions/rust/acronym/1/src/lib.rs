pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("-", " ")
        .split(" ")
        .filter(|&word| word != "")
        .map(|word| {
            let clean_word: String = word.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
            if clean_word.chars().all(|c| c.is_ascii_uppercase()) {
                clean_word.chars().nth(0).unwrap().to_string()
            } else {
                let mut string = format!(
                    "{}",
                    clean_word.chars().nth(0).unwrap().to_ascii_uppercase()
                );
                clean_word
                    .chars()
                    .skip(1)
                    .filter(|c| c.is_ascii_uppercase())
                    .for_each(|c| string.push(c));
                string
            }
        })
        .collect()
}
