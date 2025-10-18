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
    match (len as f64).sqrt() - len.isqrt() as f64 {
        0.0 => (len.isqrt(), len.isqrt()),
        n if n < 0.5 => (len.isqrt(), len.isqrt() + 1),
        n if n > 0.5 => (len.isqrt() + 1, len.isqrt() + 1),
        _ => unreachable!(),
    }
}
