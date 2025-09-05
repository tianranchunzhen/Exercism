/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        Some(
            (0..s1.len())
                .filter(|&i| s1.chars().nth(i) != s2.chars().nth(i))
                .count(),
        )
    }
}