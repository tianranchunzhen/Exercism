pub fn reply(message: &str) -> &str {
    let if_yell =
        message.chars().any(|a| a.is_ascii_alphabetic()) && message.to_uppercase() == message;
    match message.trim() {
        "" => "Fine. Be that way!",
        m if m.ends_with("?") && if_yell => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        _ if if_yell => "Whoa, chill out!",
        _ => "Whatever.",
    }
}