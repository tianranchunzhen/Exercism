pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut n = n;
    while n != 1 {
        for i in 2..=n {
            if n % i == 0 {
                result.push(i);
                n /= i;
                break;
            }
        }
    }
    result
}