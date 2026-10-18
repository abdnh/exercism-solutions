use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
enum OperandType {
    Number,
    Placeholder,
}

#[derive(Debug)]
struct Operand {
    value: u8,
    value_type: OperandType,
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut symbols = HashSet::new();
    let terms: Vec<Vec<char>> = input
        .split_whitespace()
        .filter_map(|term| {
            let term = term.trim();
            match term {
                "+" | "==" => return None,
                _ => {
                    symbols.extend(term.chars());
                    return Some(term.chars().collect::<Vec<_>>());
                }
            }
        })
        .collect();

    for permutation in (0u8..10).permutations(symbols.len()) {
        let assignments: HashMap<char, u8> = symbols
            .iter()
            .enumerate()
            .map(|(i, s)| (*s, permutation[i]))
            .collect();
        let mut found_solution = true;
        let mut carry = 0;
        for i in 0usize.. {
            let mut operands: Vec<Operand> = vec![];
            for j in 0..terms.len() {
                if i + 1 > terms[j].len() {
                    operands.push(Operand {
                        value: 0,
                        value_type: OperandType::Placeholder,
                    });
                    continue;
                }
                let idx = terms[j].len() - i - 1;
                let Some(letter) = terms[j].get(idx) else {
                    continue;
                };
                let number = assignments.get(letter).unwrap();
                operands.push({
                    Operand {
                        value: *number,
                        value_type: OperandType::Number,
                    }
                });
            }
            if operands.is_empty() {
                break;
            }
            let operands_sum = operands
                .iter()
                .take(operands.len() - 1)
                .map(|o| o.value as u64)
                .sum::<u64>();
            let expected_result = operands.iter().last().unwrap();
            if expected_result.value_type != OperandType::Placeholder
                && ((carry + operands_sum) % 10) as u8 != expected_result.value
            {
                found_solution = false;
                break;
            }
            carry = (carry + operands_sum) / 10;

            if expected_result.value_type == OperandType::Placeholder {
                break;
            }
        }
        if found_solution {
            if !terms
                .iter()
                .any(|t| assignments.get(t.first().unwrap()).unwrap() == &0)
            {
                return Some(assignments);
            }
        }
    }

    None
}
