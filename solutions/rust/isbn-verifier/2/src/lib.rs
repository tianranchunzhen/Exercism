pub fn is_valid_isbn(isbn: &str) -> bool {
    let cleand_isbn = isbn.replace("-", "");
    if cleand_isbn.len() != 10 {
        return false;
    }

    let sum: Option<u32> = cleand_isbn
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let value = match c {
                '0'..='9' => c.to_digit(10).unwrap(),
                'X' if i == 9 => 10,
                _ => return None,
            };
            Some(value * (10 - i as u32))
        })
        .sum();
    match sum {
        Some(s) => s % 11 == 0,
        None => false,
    }
}

