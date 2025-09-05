pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let if_yell =
        message.chars().any(|a| a.is_ascii_alphabetic()) && message.to_uppercase() == message;
    match message {
        _ if message.is_empty() => "Fine. Be that way!",
        _ if message.ends_with("?") && if_yell => "Calm down, I know what I'm doing!",
        _ if message.ends_with("?") => "Sure.",
        _ if if_yell => "Whoa, chill out!",
        _ => "Whatever.",
    }
}