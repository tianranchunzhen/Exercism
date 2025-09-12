fn letter_to_score(letter: char) -> u64 {
    match letter.to_uppercase().to_string().as_str() {
        "A" | "E" | "I" | "O" | "U" | "L" | "N" | "R" | "S" | "T" => 1,
        "D" | "G" => 2,
        "B" | "C" | "M" | "P" => 3,
        "F" | "H" | "V" | "W" | "Y" => 4,
        "K" => 5,
        "J" | "X" => 8,
        "Q" | "Z" => 10,
        _ => 0,
    }
}

pub fn score(word: &str) -> u64 {
    word.chars().map(letter_to_score).sum()
}