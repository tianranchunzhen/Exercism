use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_prime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let result = plaintext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_ascii_digit() {
                c
            } else {
                let i = (c.to_ascii_lowercase() as u8) - b'a';
                (((a * i as i32 + b) % 26) as u8 + b'a') as char
            }
        })
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");
    Ok(result)
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_prime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let Some(a_inv) = (1..26).find(|i| (a * *i) % 26 == 1) else {
        return Err(AffineCipherError::NotCoprime(a));
    };

    let result: String = ciphertext
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            if c.is_ascii_digit() {
                c
            } else {
                ((a_inv * ((c as u8 - b'a') as i32 - b)).rem_euclid(26) as u8 + b'a') as char
            }
        })
        .collect();
    Ok(result)
}

fn is_prime(a: i32, m: i32) -> bool {
    for i in 2..a.isqrt() + 1 {
        if a % i == 0 && m % i == 0 {
            return false;
        }
    }
    true
}
