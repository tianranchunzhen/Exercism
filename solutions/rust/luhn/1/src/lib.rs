pub fn is_valid(code: &str) -> bool {
    let new_code = code.replace(" ", "").chars().rev().collect::<String>();
    if new_code.len() <= 1 {return false;}
    let mut series = Vec::new();
    for (i, char) in new_code.chars().enumerate() {
        if let Some(n) = char.to_digit(10) {
            series.push(match i {
                _ if i % 2 == 0 => n,
                _ => match n {
                    _ if n * 2 > 9 => n * 2 -9,
                    _ => n * 2,
                }
            });
        } else {
            return false;
        }
    }
    series.iter().sum::<u32>() % 10 == 0
}