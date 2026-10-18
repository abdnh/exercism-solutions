use std::collections::{HashMap, HashSet};

fn get_char_frequencies(word: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for chr in word.chars() {
        map.entry(chr).and_modify(|c| *c += 1).or_insert(1);
    }
    map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_chars = get_char_frequencies(&word);
    possible_anagrams
        .iter()
        .copied()
        .filter(|candidate| {
            let candidate = candidate.to_lowercase();
            if candidate == word {
                false
            } else {
                let chars = get_char_frequencies(&candidate);
                word_chars == chars
            }
        })
        .collect()
}
