use regex::Regex;
use once_cell::sync::Lazy;

static RULE1: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(a|e|i|o|u|xr|yt)").unwrap());
static RULE2: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([^aeiou]+)").unwrap());
static RULE3: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([^aeiou]*qu)").unwrap());
static RULE4: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([^aeiou]+)(y.*)").unwrap());

pub fn translate(input: &str) -> String {
    input
        .split(' ')
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    // Rule 1: Starts with a vowel sound
    if RULE1.is_match(word) {
        return format!("{}ay", word);
    }

    // Rule 3: Starts with consonant(s) followed by "qu"
    if let Some(mat) = RULE3.find(word) {
        let (consonants_and_qu, rest) = word.split_at(mat.end());
        return format!("{}{}ay", rest, consonants_and_qu);
    }

    // Rule 4: Starts with consonant(s) followed by "y"
    if let Some(caps) = RULE4.captures(word) {
        let consonants = &caps[1];
        let rest = &caps[2];
        return format!("{}{}ay", rest, consonants);
    }

    // Rule 2: Starts with consonant(s)
    if let Some(mat) = RULE2.find(word) {
        let (consonants, rest) = word.split_at(mat.end());
        return format!("{}{}ay", rest, consonants);
    }

    word.to_string()
}
