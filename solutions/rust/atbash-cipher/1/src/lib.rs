/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let bytes: Vec<u8> = plain
        .replace(|c: char| !c.is_alphanumeric(), "")
        .as_bytes()
        .chunks(5)
        .flat_map(|chunk| {
            let mut v = chunk
                .iter()
                .copied()
                .map(|c| {
                    if c.is_ascii_alphabetic() {
                        let c = c.to_ascii_lowercase();
                        let idx = 'z' as u8 - c;

                        'a' as u8 + idx
                    } else {
                        c
                    }
                })
                .collect::<Vec<_>>();
            v.push(b' ');

            v
        })
        .collect();

    String::from_utf8_lossy(bytes.as_slice()).trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| {
            if c.is_whitespace() {
                None
            } else if c.is_alphabetic() {
                let c = c.to_ascii_lowercase();
                let idx = 'z' as u8 - c as u8;

                Some(('a' as u8 + idx) as char)
            } else {
                Some(c)
            }
        })
        .collect()
}
