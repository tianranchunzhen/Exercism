pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut candidates = 2..;
    while n > 1 {
        let candidate = candidates.next().unwrap();
        while n % candidate == 0 {
            n /= candidate;
            factors.push(candidate);
        }
    }
    factors
}