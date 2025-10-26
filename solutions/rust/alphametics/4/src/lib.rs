use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .unique()
        .collect_vec();

    let first_letters = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .map(|word| word.chars().next().unwrap())
        .collect_vec();

    if letters.len() > 10 {
        return None;
    }

    (0_u8..=9_u8)
        .combinations(letters.len())
        .flat_map(|combination| combination.into_iter().permutations(letters.len()))
        .par_bridge()
        .map(|permutation| letters.iter().cloned().zip(permutation).collect())
        .find_any(|alpha_map: &HashMap<char, u8>| {
            first_letters.iter().any(|c| alpha_map.get(c) == Some(&0)) && test_solution(input, alpha_map)
        })
}

pub fn test_solution(input: &str, alpha_map: &HashMap<char, u8>) -> bool {
    let (left, result) = input.split_once(" == ").unwrap();
    let nums = left
        .split(" + ")
        .map(|num| translate(num, alpha_map))
        .sum::<usize>();
    nums == translate(result, alpha_map)
}

fn translate(str_num: &str, alpha_map: &HashMap<char, u8>) -> usize {
    str_num.chars().fold(0, |acc, c| {
        let digit = alpha_map.get(&c).unwrap();
        acc * 10 + *digit as usize
    })
}
