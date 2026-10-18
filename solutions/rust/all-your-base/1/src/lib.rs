use std::{
    cell::{Cell, RefCell},
    u32,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    // Convert to decimal first
    let mut invalid_digit = None;
    let mut decimal = number
        .iter()
        .copied()
        .rev()
        .enumerate()
        .map(|(i, d)| {
            if d >= from_base && invalid_digit.is_none() {
                invalid_digit = Some(d);
            }
            d * from_base.pow(i as u32)
        })
        .sum::<u32>();
    if let Some(d) = invalid_digit {
        return Err(Error::InvalidDigit(d));
    }

    // Convert to to_base
    let mut digits = vec![];
    loop {
        if decimal != to_base - 1 {
            let d = decimal % to_base;
            decimal /= to_base;
            digits.push(d);
            if decimal == 0 {
                break;
            }
        } else {
            digits.push(to_base - 1);
            break;
        };
    }
    digits.reverse();

    Ok(digits)
}
