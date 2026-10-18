use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut histogram: HashMap<char, usize> = HashMap::new();
    for char in candidate.chars() {
        if char != ' ' && char != '-' {
            histogram
                .entry(char.to_ascii_lowercase())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    !histogram.values().any(|c| *c > 1)
}
