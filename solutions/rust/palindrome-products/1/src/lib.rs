use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    min: u64,
    max: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        (self.min..self.value.isqrt() + 1)
            .filter(|i| self.value % i == 0 && self.value / i <= self.max)
            .map(|i| (i, self.value / i))
            .collect()
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes = (min * min..=max * max)
        .map(|n| n.to_string())
        .filter(|n| n.chars().eq(n.chars().rev()))
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();
    palindromes.sort_unstable();
    let smallest = palindromes
        .iter()
        .find(|&&n| !Palindrome { value: n, min, max }.into_factors().is_empty())
        .cloned();
    let largest = palindromes
        .iter()
        .rev()
        .find(|&&n| !Palindrome { value: n, min, max }.into_factors().is_empty())
        .cloned();

    match (smallest, largest) {
        (Some(s), Some(l)) => Some((
            Palindrome { value: s, min, max },
            Palindrome { value: l, min, max },
        )),
        _ => None,
    }
}
