pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..digits.len())
        .filter_map(|i| {
            if i + len > digits.len() {
                None
            } else {
                let substring = &digits[i..i + len];
                Some(substring.to_string())
            }
        })
        .collect()
}
