pub fn actions(n: u8) -> Vec<&'static str> {
    let mut res = Vec::new();
    for (i, num) in format!("{n:05b}").chars().rev().enumerate() {
        match i {
            0 if num == '1' => { res.push("wink"); }
            1 if num == '1' => { res.push("double blink"); }
            2 if num == '1' => { res.push("close your eyes"); }
            3 if num == '1' => { res.push("jump"); }
            4 if num == '1' => { res = res.into_iter().rev().collect(); }
            _ => (),
        }
    }
    res
}
