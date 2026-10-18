use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation() && c != '\'')
        .fold(HashMap::new(), |mut acc, word| {
            let word = word.trim_matches(|c| c == '\'').to_lowercase();
            if !word.is_empty() {
                acc.entry(word).and_modify(|c| *c += 1).or_insert(1);
            }
            acc
        })
}
