pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut step_count = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2
        } else {
            n = n * 3 + 1
        }
        step_count += 1
    }
    Some(step_count)
}
