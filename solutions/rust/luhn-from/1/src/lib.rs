use std::fmt::Display;

pub struct Luhn {
    digits: Vec<u8>,
    passed_initial_validation: bool,
}

impl Luhn {
    fn new(digits: Vec<u8>, passed_initial_validation: bool) -> Self {
        Self {
            digits,
            passed_initial_validation,
        }
    }

    pub fn is_valid(&self) -> bool {
        if !self.passed_initial_validation {
            return false;
        }

        let mut zero_count = 0;
        let sum = self
            .digits
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        let mut passed_initial_validation = true;
        let digits: Vec<u8> = input
            .to_string()
            .chars()
            .filter_map(|c| {
                if c != ' ' && !c.is_ascii_digit() {
                    passed_initial_validation = false;
                }
                c.to_digit(10).map(|d| d as u8)
            })
            .collect();

        Self::new(digits, passed_initial_validation)
    }
}
