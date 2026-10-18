#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    if let Some(c) = string_digits.chars().find(|c| !c.is_ascii_digit()) {
        return Err(Error::InvalidDigit(c));
    }

    Ok((0..string_digits.len())
        .filter_map(|i| {
            let end = if i + span >= string_digits.len() {
                string_digits.len()
            } else {
                i + span
            };
            let series = &string_digits[i..end];
            if series.len() == span {
                Some(
                    series
                        .chars()
                        .filter_map(|c| c.to_digit(10).map(|d| d as u64))
                        .product(),
                )
            } else {
                None
            }
        })
        .max()
        .unwrap_or(1))
}
