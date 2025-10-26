use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn get_weights(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut weights = HashMap::new();
    let (addends, result) = input.split_once(" == ").unwrap();

    // Process the numbers being added (positive weights)
    for term in addends.split(" + ") {
        for (i, c) in term.chars().rev().enumerate() {
            *weights.entry(c).or_insert(0) += 10i64.pow(i as u32);
        }
    }

    // Process the result (negative weights)
    for (i, c) in result.chars().rev().enumerate() {
        *weights.entry(c).or_insert(0) -= 10i64.pow(i as u32);
    }

    // Heuristic: sort by weight magnitude to prune search space faster
    let mut sorted_weights = weights.into_iter().collect::<Vec<_>>();
    sorted_weights.sort_by_key(|&(_, w)| -w.abs());

    sorted_weights.into_iter().unzip()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let first_letters: HashSet<char> = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .map(|word| word.chars().next().unwrap())
        .collect();

    let (letters, weights) = get_weights(input);

    if letters.len() > 10 {
        return None;
    }

    for p in (0..=9).permutations(letters.len()) {
        // Check for leading zeros first, it's a cheap operation.
        if p.iter()
            .zip(letters.iter())
            .any(|(&digit, &letter)| digit == 0 && first_letters.contains(&letter))
        {
            continue;
        }

        // Use dot product of permutations and weights to check for solution
        let sum: i64 = p
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| (d as i64) * w)
            .sum();

        if sum == 0 {
            let solution = letters
                .iter()
                .zip(p.iter())
                .map(|(&l, &d)| (l, d))
                .collect();
            return Some(solution);
        }
    }

    None
}
