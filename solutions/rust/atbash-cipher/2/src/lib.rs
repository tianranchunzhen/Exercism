use itertools::Itertools;

pub fn encode(plain: &str) -> String {
    atbash(plain)
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    atbash(cipher).collect()
}

fn atbash(text: &str) -> impl Iterator<Item = char> {
    text.chars().filter(|c| c.is_ascii_alphanumeric()).map(|c| {
        if c.is_ascii_alphabetic() {
            (b'z' - (c.to_ascii_lowercase() as u8 - b'a')) as char
        } else {
            c
        }
    })
}
