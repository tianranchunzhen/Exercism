pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for s in string.chars() {
        if s == '(' || s == '[' || s == '{' {
            stack.push(s);
            continue;
        }
        match s {
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => continue,
        }
    }
    stack.is_empty()
}