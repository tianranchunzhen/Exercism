pub fn is_armstrong_number(num: u32) -> bool {
    let stred_num = num.to_string();
    let power = stred_num.len() as u32;

    let mut res = 0;
    for i in stred_num.chars() {
        res += i.to_digit(10).unwrap().pow(power);
    }
    if res == num { true } else { false }
}