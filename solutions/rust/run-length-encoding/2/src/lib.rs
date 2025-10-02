pub fn encode(source: &str) -> String {
    let mut res = String::new();
    let mut iter = source.chars().peekable();
    let mut current_count = 0;
    while let Some(c) = iter.next() {
        current_count += 1;
        if Some(&c) != iter.peek() {
            if current_count > 1 {
                res.push_str(&current_count.to_string())
            }
            res.push(c);
            current_count = 0;
        }
    }
    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let mut num_str = String::new();
    for c in source.chars() {
        if c.is_ascii_digit() {
            num_str.push(c);
        } else {
            let c_num = num_str.parse::<usize>().unwrap_or(1);
            res.push_str(&c.to_string().repeat(c_num));
            num_str.clear();
        }
    }
    res
}
