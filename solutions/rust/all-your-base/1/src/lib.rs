#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 0 || from_base == 1 {
        return Err(Error::InvalidInputBase);
    } else if to_base == 0 || to_base == 1 {
        return Err(Error::InvalidOutputBase);
    }

    for &n in number {
        if n >= from_base {
            return Err(Error::InvalidDigit(n));
        }
    }
    let mut ten_base_number: u32 = number
        .iter()
        .enumerate()
        .map(|(i, n)| n * from_base.pow((number.len() - i - 1) as u32))
        .sum();

    let mut res = Vec::new();
    while ten_base_number > 0 {
        res.push(ten_base_number % to_base);
        ten_base_number /= to_base;
    }
    if res.is_empty() {
        res.push(0);
    } else {
        res.reverse();
    }
    Ok(res)
}
