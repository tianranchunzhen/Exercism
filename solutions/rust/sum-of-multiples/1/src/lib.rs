pub use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut hashmap = HashSet::new();
    for factor in factors {
        if *factor == 0 { continue; }
        hashmap.extend((*factor..limit).filter(|x| x % factor == 0));
    }
    hashmap.iter().sum()
}