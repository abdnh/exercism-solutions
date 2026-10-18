/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut digit_count = 0;
    let mut sum = 0;
    for char in isbn.chars() {
        if let Some(digit) = char.to_digit(10) {
            sum += (10 - digit_count) * digit;
            digit_count += 1;
        } else if char.to_ascii_lowercase() == 'x' && digit_count == 9 {
            sum += 10;
            digit_count += 1;
        } else if char != '-' {
            return false;
        }
    }
    if digit_count != 10 {
        return false;
    }

    sum % 11 == 0
}
