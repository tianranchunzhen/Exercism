use itertools::Itertools;

pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| get_replaced(c.to_ascii_lowercase()))
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| get_replaced(c.to_ascii_lowercase()))
        .collect()
}

fn get_replaced(c: char) -> char {
    if c.is_ascii_digit() {
        c
    } else {
        (219 - (c as u8)) as char
    }
}
