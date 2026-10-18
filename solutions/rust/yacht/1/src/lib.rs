use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

fn sum_number(dice: &Dice, number: u8) -> u8 {
    dice.iter().copied().filter(|d| *d == number).sum()
}

fn histogram(dice: &Dice) -> HashMap<u8, u8> {
    dice.iter().fold(HashMap::new(), |mut acc, x| {
        acc.entry(*x).and_modify(|v| *v += 1).or_insert(1);
        acc
    })
}

fn is_straight(dice: &mut Dice, offset: usize) -> bool {
    dice.sort();
    dice.iter()
        .enumerate()
        .all(|(i, v)| *v as usize == i + offset)
}

pub fn score(mut dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => sum_number(&dice, 1),
        Category::Twos => sum_number(&dice, 2),
        Category::Threes => sum_number(&dice, 3),
        Category::Fours => sum_number(&dice, 4),
        Category::Fives => sum_number(&dice, 5),
        Category::Sixes => sum_number(&dice, 6),
        Category::FullHouse => {
            let mut map = histogram(&dice);
            let mut values: Vec<_> = map.values().copied().collect();
            values.sort();
            if values.as_slice() == [2, 3] {
                map.drain().map(|(k, v)| k * v).sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            let map = histogram(&dice);
            let max = map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
            if *max.1 >= 4 { max.0 * 4 } else { 0 }
        }
        Category::LittleStraight => {
            if is_straight(&mut dice, 1) {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if is_straight(&mut dice, 2) {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            let first = dice[0];
            if dice.iter().all(|v| *v == first) {
                50
            } else {
                0
            }
        }
    }
}
