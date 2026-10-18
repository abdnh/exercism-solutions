pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { 'a' } else { 'A' };
                let idx = c as u8 - base as u8;
                let new_idx = (idx + key) % 26;
                (base as u8 + new_idx) as char
            } else {
                c
            }
        })
        .collect()
}
