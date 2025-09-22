use itertools::Itertools;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|(i, j)| {
            input[*i].iter().max().unwrap() == &input[*i][*j]
                && input.iter().map(|row| row[*j]).min().unwrap() == input[*i][*j]
        })
        .collect()
}