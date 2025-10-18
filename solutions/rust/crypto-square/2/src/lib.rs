pub fn encrypt(input: &str) -> String {
    let text = input
        .chars()
        .filter(|char| char.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    let (row, col) = calc_rc(text.len());
    (0..col)
        .map(|c| text.chars().skip(c).step_by(col).collect::<String>())
        .map(|chunk| format!("{chunk:<row$}"))
        .collect::<Vec<String>>()
        .join(" ")
}

fn calc_rc(len: usize) -> (usize, usize) {
    let n = len.isqrt();
    if n * n == len {
        (n, n)
    } else if n * (n + 1) >= len {
        (n, n + 1)
    } else {
        (n + 1, n + 1)
    }
}
