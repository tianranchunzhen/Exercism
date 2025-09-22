pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=upper_bound.isqrt() + 1 {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= upper_bound {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &a)| if a { Some(i as u64) } else { None })
        .collect()
}