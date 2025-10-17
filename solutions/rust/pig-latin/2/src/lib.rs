use regex::Regex;

pub fn translate(input: &str) -> String {
    input
        .split(' ')
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    // Rule 1: Starts with a vowel sound
    let re = Regex::new(r"^(a|e|i|o|u|xr|yt)").unwrap();
    if re.is_match(word) {
        return format!("{}ay", word);
    }

    // Rule 3: Starts with consonant(s) followed by "qu"
    let re = Regex::new(r"^([^aeiou]*qu)").unwrap();
    if let Some(mat) = re.find(word) {
        let (consonants_and_qu, rest) = word.split_at(mat.end());
        return format!("{}{}ay", rest, consonants_and_qu);
    }

    // Rule 4: Starts with consonant(s) followed by "y"
    let re = Regex::new(r"^([^aeiou]+)(y.*)").unwrap();
    if let Some(caps) = re.captures(word) {
        let consonants = &caps[1];
        let rest = &caps[2];
        return format!("{}{}ay", rest, consonants);
    }

    // Rule 2: Starts with consonant(s)
    let re = Regex::new(r"^([^aeiou]+)").unwrap();
    if let Some(mat) = re.find(word) {
        let (consonants, rest) = word.split_at(mat.end());
        return format!("{}{}ay", rest, consonants);
    }

    word.to_string()
}
