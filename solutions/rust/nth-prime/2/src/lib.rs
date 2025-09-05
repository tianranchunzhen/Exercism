pub fn is_prime(n: u32) -> bool {
    (2..n.isqrt() + 1).all(|a| n % a != 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|n| is_prime(*n)).nth(n as usize).unwrap()
}