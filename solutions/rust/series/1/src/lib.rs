pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return Vec::new();
    }

    let mut res_vec: Vec<String> = vec![];
    for index in 0..(digits.len() - len + 1) {
        res_vec.push(digits[index..index + len].to_string());
    }
    res_vec
}