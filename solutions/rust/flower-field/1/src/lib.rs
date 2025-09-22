use itertools::Itertools;

fn get_around_chars(garden: &[&str], idx_row: usize, idx_col: usize) -> usize {
    let min_idx_row = if idx_row == 0 { 0 } else { idx_row - 1 };
    let min_idx_col = if idx_col == 0 { 0 } else { idx_col - 1 };
    (min_idx_row..=idx_row + 1)
        .cartesian_product(min_idx_col..=idx_col + 1)
        .filter_map(|(i, j)| garden.get(i)?.chars().nth(j))
        .filter(|&char| char == '*')
        .count()
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    // let n_row = garden.len();
    match garden.len() {
        0 => Vec::new(),
        n_row => (0..n_row)
            .map(|i| {
                (0..garden[0].len())
                    .map(|j| match garden[i].chars().nth(j).unwrap() {
                        '*' => '*',
                        _ => match get_around_chars(garden, i, j) {
                            0 => ' ',
                            count => (b'0' + count as u8) as char,
                        },
                    })
                    .collect::<String>()
            })
            .collect(),
    }
}
