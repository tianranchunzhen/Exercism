#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    if span == 0 {
        return Ok(1);
    }

    let mut digit_series = Vec::new();
    for char in string_digits.chars() {
        if char.to_digit(10).is_none() {
            return Err(Error::InvalidDigit(char));
        } else {
            digit_series.push(char.to_digit(10).unwrap() as u64);
        }
    }

    let max = digit_series
        .windows(span)
        .map(|a| a.iter().product())
        .max()
        .unwrap();
    Ok(max)
}
