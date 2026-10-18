pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    if message.chars().all(|c| {
        c.is_uppercase() || c.is_ascii_punctuation() || c.is_whitespace() || c.is_numeric()
    }) && message.chars().any(|c| c.is_uppercase())
    {
        if message.chars().last().unwrap() == '?' {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    }
    if message.chars().last().unwrap() == '?' {
        return "Sure.";
    }

    return "Whatever.";
}
