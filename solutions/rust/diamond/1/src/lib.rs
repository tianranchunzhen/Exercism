pub fn get_diamond(c: char) -> Vec<String> {
    let letters: Vec<char> = ('A'..=c).collect();
    let diamond_lower: Vec<String> = letters
        .iter()
        .enumerate()
        .map(|(i, letter)| {
            let half_line = format!(
                "{}{}{}",
                " ".repeat(letters.len() - i - 1),
                *letter,
                " ".repeat(i)
            );
            format!(
                "{}{}",
                half_line,
                half_line.chars().rev().skip(1).collect::<String>()
            )
        })
        .collect();
    diamond_lower
        .iter()
        .chain(diamond_lower.iter().rev().skip(1))
        .cloned()
        .collect()
}