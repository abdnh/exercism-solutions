use std::iter::Sum;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use itertools::EitherOrBoth;
use itertools::Itertools;
use regex_lite::Regex;
use std::cmp::Ordering;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Eq, Ord, Clone)]
pub struct Decimal {
    digits: Vec<u8>,
    fraction_start: usize,
    sign: i8,
}

fn without_leading_zeroes(digits: &[u8]) -> &[u8] {
    if let Some(last) = digits
        .iter()
        .copied()
        .take_while(|d| *d == 0)
        .enumerate()
        .last()
    {
        &digits[last.0 + 1..]
    } else {
        digits
    }
}

fn without_trailing_zeroes(digits: &[u8]) -> &[u8] {
    if let Some(last) = digits
        .iter()
        .copied()
        .rev()
        .take_while(|d| *d == 0)
        .enumerate()
        .last()
    {
        &digits[..digits.len() - last.0 - 1]
    } else {
        digits
    }
}

impl Decimal {
    fn new(mut digits: Vec<u8>, fraction_start: usize, sign: i8) -> Self {
        if fraction_start > digits.len() {
            digits.extend((0..fraction_start - digits.len()).map(|_| 0));
        }
        let mut integer = without_leading_zeroes(&digits[..fraction_start]).to_vec();
        let mut fraction = without_trailing_zeroes(&digits[fraction_start..]).to_vec();
        if integer.is_empty() {
            integer.push(0);
        }
        if fraction.is_empty() {
            fraction.push(0);
        }
        let fraction_start = integer.len();
        let digits: Vec<_> = integer.iter().chain(fraction.iter()).copied().collect();
        Self {
            digits,
            fraction_start,
            sign,
        }
    }

    fn integer(&self) -> &[u8] {
        &self.digits[..self.fraction_start]
    }

    fn signed_integer(&self) -> Box<dyn Iterator<Item = i8> + '_> {
        Box::new(
            self.digits[..self.fraction_start]
                .iter()
                .map(|d| *d as i8 * self.sign),
        )
    }

    fn fraction(&self) -> &[u8] {
        &self.digits[self.fraction_start..]
    }

    fn signed_fraction(&self) -> Box<dyn Iterator<Item = i8> + '_> {
        Box::new(
            self.digits[self.fraction_start..]
                .iter()
                .map(|d| *d as i8 * self.sign),
        )
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        let re = Regex::new("(?P<sign>(\\+|-))?(?P<integer>\\d+)((.)(?P<decimal>\\d+))?").unwrap();
        let Some(caps) = re.captures(input) else {
            return None;
        };
        let sign = caps
            .name("sign")
            .map(|c| if c.as_str() == "+" { 1 } else { -1 })
            .unwrap_or(1);
        let Some(integer) = caps.name("integer") else {
            return None;
        };
        let mut digits: Vec<u8> = integer
            .as_str()
            .chars()
            .map(|c| c as u8 - '0' as u8)
            .collect();
        let fraction_start = digits.len();
        let decimal: Vec<u8> = if let Some(decimal) = caps.name("decimal") {
            decimal
                .as_str()
                .chars()
                .map(|c| c as u8 - '0' as u8)
                .collect()
        } else {
            Vec::new()
        };

        digits.extend(decimal);

        Some(Decimal::new(digits, fraction_start, sign))
    }

    fn abs(&self) -> Self {
        let mut out = self.clone();
        out.sign = 1;

        out
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.fraction_start == other.fraction_start && self.digits == other.digits
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (decimal1, decimal2) = normalize_operands(self.clone(), other.clone());

        Some(
            decimal1
                .fraction_start
                .cmp(&decimal2.fraction_start)
                .then_with(|| decimal1.signed_integer().cmp(decimal2.signed_integer()))
                .then_with(|| decimal1.signed_fraction().cmp(decimal2.signed_fraction())),
        )
    }
}

/// Normalize the given decimals to have the same number of integer and fractional digits
pub fn normalize_operands(mut decimal1: Decimal, mut decimal2: Decimal) -> (Decimal, Decimal) {
    let (mut integer1, mut integer2) = decimal1
        .integer()
        .iter()
        .rev()
        .copied()
        .zip_longest(decimal2.integer().iter().rev().copied())
        .map(|item| match item {
            EitherOrBoth::Both(a, b) => (a, b),
            EitherOrBoth::Left(a) => (a, 0),
            EitherOrBoth::Right(b) => (0, b),
        })
        .fold((vec![], vec![]), |mut v: (Vec<_>, Vec<_>), pair| {
            v.0.push(pair.0);
            v.1.push(pair.1);

            v
        });
    integer1.reverse();
    integer2.reverse();

    let (fraction1, fraction2) = decimal1
        .fraction()
        .iter()
        .copied()
        .zip_longest(decimal2.fraction().iter().copied())
        .map(|item| match item {
            EitherOrBoth::Both(a, b) => (a, b),
            EitherOrBoth::Left(a) => (a, 0),
            EitherOrBoth::Right(b) => (0, b),
        })
        .fold((vec![], vec![]), |mut v: (Vec<_>, Vec<_>), pair| {
            v.0.push(pair.0);
            v.1.push(pair.1);

            v
        });

    decimal1.fraction_start = integer1.len();
    decimal1.digits = integer1
        .iter()
        .copied()
        .chain(fraction1.iter().copied())
        .collect();
    decimal2.fraction_start = integer2.len();
    decimal2.digits = integer2
        .iter()
        .copied()
        .chain(fraction2.iter().copied())
        .collect();

    (decimal1, decimal2)
}

fn carry_and_remainder(sum: i8) -> (i8, i8) {
    if (sum >= 0 && sum <= 9) || (sum <= 0 && sum >= -9) {
        (0, sum)
    } else {
        (sum / 10, sum % 10)
    }
}
impl Neg for Decimal {
    type Output = Decimal;
    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        if self.sign == 1 {
            out.sign = -1;
        } else {
            out.sign = 1;
        }

        out
    }
}

impl Neg for &Decimal {
    type Output = Decimal;
    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        if self.sign == 1 {
            out.sign = -1;
        } else {
            out.sign = 1;
        }

        out
    }
}

impl Add for Decimal {
    type Output = Decimal;
    fn add(self, rhs: Self) -> Self::Output {
        let (mut decimal1, mut decimal2) = normalize_operands(self, rhs);

        // Swap numbers for borrow
        if decimal2 > decimal1 {
            (decimal1, decimal2) = (decimal2, decimal1);
        }
        let negated = if decimal2.abs() > decimal1.abs() {
            (decimal1, decimal2) = (decimal2.abs(), -decimal1);
            true
        } else {
            false
        };

        let fraction_len = decimal1.fraction().len();
        let mut sign = 1;
        let mut carry = 0;
        let mut borrow = 0;
        let mut digits: Vec<_> = decimal1
            .digits
            .iter()
            .copied()
            .enumerate()
            .rev()
            .map(|(i, d1)| {
                let d2 = decimal2.digits[i];
                let mut sum = if d1 != 0 && borrow != 0 {
                    borrow = 0;
                    carry + decimal1.sign * (d1 - 1) as i8 + decimal2.sign * d2 as i8
                } else {
                    borrow + carry + decimal1.sign * d1 as i8 + decimal2.sign * d2 as i8
                };
                if sum < 0 {
                    sum = 10 + carry + decimal2.sign * d2 as i8;
                    borrow = 9;
                }
                let (c, sum) = carry_and_remainder(sum);
                carry = c;
                sign = if sum.signum() != 0 { sum.signum() } else { 1 };

                sum.abs() as u8
            })
            .collect();
        digits.reverse();
        let digits_len = digits.len();

        let out = Decimal::new(digits, digits_len - fraction_len, sign);

        if negated { -out } else { out }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, mut rhs: Self) -> Self::Output {
        if rhs.sign == -1 {
            rhs.sign = 1;
        } else {
            rhs.sign = -1;
        }
        self.add(rhs)
    }
}

impl Sum for Decimal {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut out = Self {
            digits: vec![],
            fraction_start: 0,
            sign: 1,
        };
        for item in iter {
            out = out.clone() + item;
        }

        out
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (decimal1, decimal2) = normalize_operands(self, rhs);
        let mut sign = 1;
        let total = decimal1
            .digits
            .iter()
            .copied()
            .enumerate()
            .rev()
            .map(|(i, d1)| {
                let mut carry = 0;
                let mut digits: Vec<_> = decimal2
                    .digits
                    .iter()
                    .copied()
                    .rev()
                    .map(|d2| {
                        let (c, sum) = carry_and_remainder(
                            carry + decimal1.sign * d1 as i8 * decimal2.sign * d2 as i8,
                        );
                        carry = c;
                        sign = if sum.signum() != 0 { sum.signum() } else { 1 };

                        sum.abs() as u8
                    })
                    .collect();
                let mut fraction_start = decimal1.fraction_start;
                if i >= fraction_start {
                    fraction_start -= i - fraction_start + 1;
                } else {
                    fraction_start += fraction_start - i - 1;
                }
                digits.reverse();
                let decimal = Decimal::new(digits, fraction_start, sign);
                decimal
            })
            .sum();

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_zeroes() {
        assert_eq!(without_leading_zeroes(&[0]), &[]);
        assert_eq!(without_leading_zeroes(&[1]), &[1]);
        assert_eq!(without_leading_zeroes(&[0, 0, 1, 0, 1, 0]), &[1, 0, 1, 0]);
        assert_eq!(without_trailing_zeroes(&[0]), &[]);
        assert_eq!(without_trailing_zeroes(&[1]), &[1]);
        assert_eq!(
            without_trailing_zeroes(&[0, 0, 1, 0, 1, 0]),
            &[0, 0, 1, 0, 1]
        );
    }

    #[test]
    fn test_carry_and_remainder() {
        assert_eq!(carry_and_remainder(1), (0, 1));
        assert_eq!(carry_and_remainder(9), (0, 9));
        assert_eq!(carry_and_remainder(10), (1, 0));
        assert_eq!(carry_and_remainder(11), (1, 1));

        assert_eq!(carry_and_remainder(-1), (0, -1));
        assert_eq!(carry_and_remainder(-9), (0, -9));
        assert_eq!(carry_and_remainder(-10), (-1, 0));
        assert_eq!(carry_and_remainder(-11), (-1, -1));
    }
}
