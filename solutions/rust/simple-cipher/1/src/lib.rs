use rand::distr::{SampleString, Uniform};

fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let key = key.as_bytes();
    Some(
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_ascii_alphabetic() {
                    let idx = c as u8 - 'a' as u8;
                    let shift = key[i % key.len()] as u8 - 'a' as u8;
                    let new_idx = (idx + shift) % 26;
                    ('a' as u8 + new_idx) as char
                } else {
                    c
                }
            })
            .collect(),
    )
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let key = key.as_bytes();
    Some(
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_ascii_alphabetic() {
                    let idx = c as i16 - 'a' as i16;
                    let shift = key[i % key.len()] as i16 - 'a' as i16;
                    let new_idx = (idx - shift).rem_euclid(26) as u8;
                    ('a' as u8 + new_idx) as char
                } else {
                    c
                }
            })
            .collect(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::rng();
    let uniform = Uniform::new('a', 'z').unwrap();
    let key = uniform.sample_string(&mut rng, 100);
    let encoded = encode(&key, s).unwrap();

    (key, encoded)
}
