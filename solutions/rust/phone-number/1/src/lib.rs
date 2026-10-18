pub fn number(user_number: &str) -> Option<String> {
    let user_number =
        user_number.replace(|c: char| c.is_whitespace() || c.is_ascii_punctuation(), "");
    let user_number = user_number.trim_start_matches('1');
    (user_number.len() == 10
        && user_number.chars().enumerate().all(|(i, c)| {
            if i == 0 || i == 3 {
                !&['0', '1'].contains(&c)
            } else {
                c.is_ascii_digit()
            }
        }))
    .then_some(user_number.into())
}
