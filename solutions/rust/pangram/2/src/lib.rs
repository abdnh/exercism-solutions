use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter_map(|c| c.is_ascii_alphabetic().then_some(c.to_ascii_lowercase()))
        .collect::<HashSet<_>>()
        .len()
        == 26
}
