/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut invalid = false;
    let mut zero_count = 0;
    let sum = code
        .chars()
        .filter_map(|c| {
            if c != ' ' && !c.is_ascii_digit() {
                invalid = true;
            }
            c.to_digit(10)
        })
        .rev()
        .enumerate()
        .map(|(i, d)| {
            if d == 0 {
                zero_count += 1;
            }
            if i % 2 == 1 {
                let d = d * 2;
                if d > 9 {
                    return d - 9;
                }
                return d;
            }
            return d;
        })
        .sum::<u32>();
    !invalid && (sum != 0 || zero_count != 1) && sum % 10 == 0
}
