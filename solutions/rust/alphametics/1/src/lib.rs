use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .unique()
        .collect_vec();

    if letters.len() > 10 {
        return None;
    }

    (0_u8..=9_u8)
        .combinations(letters.len())
        .flat_map(|combination| combination.into_iter().permutations(letters.len()))
        .map(|permutation| letters.iter().cloned().zip(permutation).collect())
        .find(|alpha_map| test_solution(input, alpha_map))
}

pub fn test_solution(input: &str, alpha_map: &HashMap<char, u8>) -> bool {
    let (left, result) = input.split_once(" == ").unwrap();
    let Some(result) = translate(result, alpha_map) else {
        return false;
    };
    let Some(nums) = left
        .split(" + ")
        .map(|num| translate(num, alpha_map))
        .collect::<Option<Vec<usize>>>()
    else {
        return false;
    };
    nums.iter().sum::<usize>() == result
}

fn translate(str_num: &str, alpha_map: &HashMap<char, u8>) -> Option<usize> {
    let first_num = str_num.chars().next().unwrap();
    if alpha_map.get(&first_num).unwrap() == &0 {
        None
    } else {
        Some(str_num.chars().fold(0, |acc, c| {
            let digit = alpha_map.get(&c).unwrap();
            acc * 10 + *digit as usize
        }))
    }
}
