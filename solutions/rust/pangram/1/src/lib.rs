use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for char in sentence.chars() {
        if char.is_ascii_alphabetic() {
            set.insert(char.to_ascii_lowercase());
        }
    }

    set.len() == 26
}
