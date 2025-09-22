pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut not_primes: Vec<u64> = Vec::new();
    (2..=upper_bound)
        .filter(|i| {
            if not_primes.contains(i) {
                false
            } else {
                not_primes.extend(
                    (i + 1..=upper_bound)
                        .filter(|n| n.is_multiple_of(*i))
                        .collect::<Vec<u64>>(),
                );
                true
            }
        })
        .collect()
}
