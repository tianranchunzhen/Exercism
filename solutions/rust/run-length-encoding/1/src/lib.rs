use itertools::Itertools;

pub fn encode(source: &str) -> String {
    let mut res = String::new();
    let mut current_count = 1;
    for (i, (x, y)) in source.chars().tuple_windows().enumerate() {
        if x == y {
            current_count += 1;
            if i == source.len() - 2 {
                res.push_str(&format!("{current_count}{x}"));
            }
        } else {
            if current_count == 1 {
                res.push(x);
            } else {
                res.push_str(&format!("{current_count}{x}"));
                current_count = 1;
            }
            if i == source.len() - 2 {
                res.push(y);
            }
        }
    }
    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let mut iter = source.chars();
    while let Some(c) = iter.next() {
        if c.is_ascii_digit() {
            let mut num_str = c.to_string();
            let mut next_c = iter.next().unwrap();
            while next_c.is_ascii_digit() {
                num_str.push(next_c);
                next_c = iter.next().unwrap();
            }
            res.push_str(
                &next_c
                    .to_string()
                    .repeat(num_str.parse::<usize>().unwrap()),
            );
        } else {
            res.push(c);
        }
    }
    res
}
