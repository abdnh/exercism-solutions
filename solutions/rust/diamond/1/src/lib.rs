pub fn get_diamond(c: char) -> Vec<String> {
    let c = c.to_ascii_uppercase();
    if !c.is_ascii_uppercase() {
        return vec![];
    }

    let letter_idx = c as u8 - b'A';
    let n = (letter_idx + 1) * 2 - 1;
    let m = n / 2;
    (0..n)
        .map(|i| {
            let symmetric_i = m - m.abs_diff(i);
            let row_char = (b'A' + symmetric_i) as char;
            (0..n)
                .map(|j| {
                    let (l, r) = (m - symmetric_i, m + symmetric_i);
                    if j == l || j == r { row_char } else { ' ' }
                })
                .collect::<String>()
        })
        .collect()
}
