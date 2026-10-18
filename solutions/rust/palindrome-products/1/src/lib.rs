use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64) -> Self {
        Self {
            value,
            factors: HashSet::new(),
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }

    pub fn add_factors(&mut self, factor1: u64, factor2: u64) {
        let mut t = [factor1, factor2];
        t.sort();
        if !self.factors.contains(&(t[1], t[0])) {
            self.factors.insert((t[0], t[1]));
        }
    }
}

fn is_palindrome(value: u64) -> bool {
    if value == 0 {
        return false;
    }
    let digits = value.ilog10() + 1;
    let mut reversed: u64 = 0;
    for digit in 0..digits {
        let v = (value / 10u64.pow(digit)) % 10;
        reversed += v * 10u64.pow(digits - digit - 1);
    }

    reversed == value
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: HashMap<u64, Palindrome> = HashMap::new();
    for i in min..=max {
        for j in min..=max {
            if is_palindrome(i * j) {
                let entry = palindromes
                    .entry(i * j)
                    .or_insert_with(|| Palindrome::new(i * j));
                entry.add_factors(j, i);
            }
        }
    }
    let min = palindromes.keys().min();
    let max = palindromes.keys().max();

    min.map(|m| (palindromes[m].clone(), palindromes[max.unwrap()].clone()))
}
