use rand::random_range;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    let key_bytes = key.bytes().map(|b| b - b'a').cycle();
    let output: String = s
        .bytes()
        .zip(key_bytes)
        .map(|(c_byte, k_byte)| ((c_byte - b'a' + k_byte) % 26) + b'a')
        .map(|byte| byte as char)
        .collect();
    Some(output)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    let key_bytes = key.bytes().map(|b| b - b'a').cycle();
    let output: String = s
        .bytes()
        .zip(key_bytes)
        .map(|(c_byte, k_byte)| ((c_byte - b'a' + 26 - k_byte) % 26) + b'a')
        .map(|byte| byte as char)
        .collect();
    Some(output)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = (0..100)
        .map(|_i| (random_range(0..26) + b'a') as char)
        .collect();
    let encode_str = encode(&key, s).unwrap();
    (key, encode_str)
}
