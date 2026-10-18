use std::{fmt::Display};

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

fn is_valid_luhn_digits(digits: Vec<u8>) -> bool {
    let mut zero_count = 0;
    let sum = digits
        .iter()
        .copied()
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
            d
        })
        .map(usize::from)
        .sum::<usize>();
    (sum != 0 || zero_count != 1) && sum % 10 == 0
}

fn is_valid_luhn_str<T: AsRef<str>>(input: T) -> bool {
    let mut passed_initial_validation = true;
    let digits: Vec<u8> = input
        .as_ref()
        .chars()
        .filter_map(|c| {
            if c != ' ' && !c.is_ascii_digit() {
                passed_initial_validation = false;
            }
            c.to_digit(10).map(|d| d as u8)
        })
        .collect();
    passed_initial_validation && is_valid_luhn_digits(digits)
}

impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid_luhn_str(self.to_string())
    }
}
