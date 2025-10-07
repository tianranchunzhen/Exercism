use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code_vec: Vec<char> = self.to_string().replace(" ", "").chars().rev().collect();
        if code_vec.len() <= 1 || code_vec.iter().any(|c| !c.is_ascii_digit()) {
            return false;
        }
        let odd_items_sum: u32 = code_vec
            .iter()
            .step_by(2)
            .map(|c| c.to_digit(10).unwrap())
            .sum();
        let even_items_sum: u32 = code_vec
            .iter()
            .skip(1)
            .step_by(2)
            .map(|c| {
                let x = c.to_digit(10).unwrap() * 2;
                if x > 9 { x - 9 } else { x }
            })
            .sum();
        (odd_items_sum + even_items_sum).is_multiple_of(10)
    }
}
