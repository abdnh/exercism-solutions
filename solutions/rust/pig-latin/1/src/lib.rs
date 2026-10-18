fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| match word {
            word if word.starts_with(is_vowel)
                || word.starts_with("xr")
                || word.starts_with("yt") =>
            {
                format!("{word}ay")
            }
            _ => {
                let trimmed_word = word.trim_start_matches(|c| !is_vowel(c));
                let consonants = &word[0..word.len() - trimmed_word.len()];
                match trimmed_word {
                    trimmed_word if consonants.ends_with('q') && trimmed_word.starts_with("u") => {
                        format!(
                            "{}{}quay",
                            &trimmed_word[1..],
                            &consonants[..consonants.len() - 1]
                        )
                    }
                    trimmed_word if consonants.contains('y') && consonants.len() > 1 => {
                        let idx = consonants.find('y').unwrap();
                        format!(
                            "{}{}{}ay",
                            &consonants[idx..],
                            trimmed_word,
                            &consonants[..idx]
                        )
                    }
                    _ => format!("{}{}ay", trimmed_word, consonants),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
