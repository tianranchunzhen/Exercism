pub fn is_valid_isbn(isbn: &str) -> bool {
    let cleand_isbn = isbn.replace("-", "");
    if cleand_isbn.len() != 10 {
        return false;
    }

    let sum: Result<u32, ()> = cleand_isbn
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let value = match c {
                '0'..='9' => c.to_digit(10).unwrap(),
                'X' if i == 9 => 10,
                _ => return Err(()),
            };
            Ok(value * (10 - i as u32))
        })
        .sum();
    match sum {
        Ok(s) => s % 11 == 0,
        Err(()) => false,
    }
}
