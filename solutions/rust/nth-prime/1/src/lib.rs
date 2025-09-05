pub fn is_prime(n: u32) -> bool {
    for i in 2..=n.isqrt() {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut candidates = 2..;
    loop {
        let x = candidates.next().unwrap();
        if is_prime(x) {
            count += 1;
        }
        if count == n + 1 {
            return x;
        }
    }
}