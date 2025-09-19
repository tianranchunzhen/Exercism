use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for a in 1..=sum / 3 {
        let sum_bc = sum - a;
        let denominator = 2 * sum_bc;
        let numerator = sum_bc * sum_bc - a * a;
        if numerator % denominator == 0 {
            let b = numerator / denominator;
            if a < b {
                triplets.insert([a, b, sum - a - b]);
            }
        }
    }

    triplets
}
