pub fn get_diamond(c: char) -> Vec<String> {
    let diamond_half: Vec<String> = ('A'..=c)
        .enumerate()
        .map(|(i, letter)| {
            let left_padding = " ".repeat(('A'..=c).count() - i - 1);
            if i == 0 {
                format!("{left_padding}{letter}{left_padding}")
            } else {
                let inner_padding = " ".repeat(i * 2 - 1);
                format!("{left_padding}{letter}{inner_padding}{letter}{left_padding}")
            }
        })
        .collect();
    diamond_half
        .iter()
        .chain(diamond_half.iter().rev().skip(1))
        .cloned()
        .collect()
}