pub fn get_diamond(c: char) -> Vec<String> {
    let letters: Vec<char> = ('A'..=c).collect();
    let diamond_half: Vec<String> = letters
        .iter()
        .enumerate()
        .map(|(i, letter)| {
            let side_padding = " ".repeat(letters.len() - i - 1);
            if i == 0 {
                format!("{}{}{}", side_padding, letter, side_padding)
            } else {
                let inner_padding = " ".repeat(2 * i - 1);
                format!(
                    "{}{}{}{}{}",
                    side_padding, letter, inner_padding, letter, side_padding
                )
            }
        })
        .collect();
    diamond_half
        .iter()
        .chain(diamond_half.iter().rev().skip(1))
        .cloned()
        .collect()
}
