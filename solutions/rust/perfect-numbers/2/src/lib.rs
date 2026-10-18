use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else {
        let sum = (1..num).filter(|i| num % i == 0).sum();
        let class = match num.cmp(&sum) {
            Ordering::Equal => Classification::Perfect,
            Ordering::Greater => Classification::Deficient,
            Ordering::Less => Classification::Abundant,
        };
        Some(class)
    }
}
