pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for s in string.chars() {
        match s {
            '(' | '[' | '{' => stack.push(s),
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
