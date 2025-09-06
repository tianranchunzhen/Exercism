use itertools::Itertools;

pub fn check(candidate: &str) -> bool {
    let candiate = candidate.replace(" ", "").replace("-", "").to_lowercase();
    candiate.chars().all(|c| c.is_ascii_alphabetic()) && candiate.chars().duplicates().count() == 0
}
