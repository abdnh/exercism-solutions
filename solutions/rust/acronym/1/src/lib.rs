pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter_map(|word| {
            let word = word.trim_matches('_');
            if word.is_empty() {
                return None;
            }
            let mut abbr_letters =
                vec![word.chars().next().unwrap().to_uppercase().next().unwrap()];
            let mut chars = word.chars().peekable();
            let mut prev = chars.next().unwrap();
            loop {
                let current = chars.next();
                let Some(c) = current else {
                    break;
                };
                let next = chars.peek();
                if let Some(n) = next
                    && n.is_lowercase()
                    && prev.is_lowercase()
                    && c.is_uppercase()
                {
                    abbr_letters.push(c);
                }
                prev = c;
            }
            Some(abbr_letters)
        })
        .flatten()
        .collect()
}
