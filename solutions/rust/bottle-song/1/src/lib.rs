use std::collections::HashMap;
pub fn upper_first(s: String) -> String {
    s[..1].to_ascii_uppercase() + &s[1..]
}
pub fn gen_bottles_text(n: u32) -> String {
    let num2str = HashMap::from([
        (10, "ten"),
        (9, "nine"),
        (8, "eight"),
        (7, "seven"),
        (6, "six"),
        (5, "five"),
        (4, "four"),
        (3, "three"),
        (2, "two"),
        (1, "one"),
        (0, "no"),
    ]);
    match n {
        1 => "one green bottle".to_string(),
        _ => format!("{} green bottles", num2str.get(&n).unwrap()),
    }
}
pub fn recite(mut start_bottles: u32, take_down: u32) -> String {
    let mut output = Vec::new();
    for _ in 0..take_down {
        output.push(format!(
            "{} hanging on the wall,\n{} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} hanging on the wall.\n",
            upper_first(gen_bottles_text(start_bottles)),
            upper_first(gen_bottles_text(start_bottles)),
            gen_bottles_text(start_bottles - 1),
        ));
        start_bottles -= 1;
    }
    output.join("\n")
}