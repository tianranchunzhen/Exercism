pub fn answer(command: &str) -> Option<i32> {
    let cleaned_command = command
        .strip_prefix("What is ")?
        .strip_suffix("?")?
        .replace("multiplied by", "multiplied-by")
        .replace("divided by", "divided-by");
    let mut iter = cleaned_command.split_whitespace();
    let mut result = iter.next()?.parse::<i32>().ok()?;
    while let Some(op) = iter.next() {
        let next_num = iter.next()?.parse::<i32>().ok()?;
        match op {
            "plus" => result += next_num,
            "minus" => result -= next_num,
            "multiplied-by" => result *= next_num,
            "divided-by" => result /= next_num,
            _ => return None,
        }
    }
    Some(result)
}
