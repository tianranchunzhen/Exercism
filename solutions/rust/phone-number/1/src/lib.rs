pub fn number(user_number: &str) -> Option<String> {
    let nums: Vec<char> = user_number.chars().filter(|c| c.is_ascii_digit()).collect();
    match nums.len() {
        10 => {
            if let ('2'..='9', '2'..='9') = (&nums[0], &nums[3]) {
                Some(nums.iter().collect())
            } else {
                None
            }
        }
        11 => {
            if let ('1', '2'..='9', '2'..='9') = (&nums[0], &nums[1], &nums[4]) {
                Some(nums.iter().skip(1).collect())
            } else {
                None
            }
        }
        _ => None
    }
}
