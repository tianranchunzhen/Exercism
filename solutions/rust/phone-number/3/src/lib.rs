use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    let digits: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();
    let re =
        Regex::new(r"^(?:1?(?P<area>[2-9]\d{2})(?P<exchange>[2-9]\d{2})(?P<subscriber>\d{4}))$")
            .unwrap();

    re.captures(&digits).map(|caps| {
        format!(
            "{}{}{}",
            &caps["area"], &caps["exchange"], &caps["subscriber"]
        )
    })
}