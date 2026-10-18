use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Unkown,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn do_operation(&self, operand1: i32, operand2: i32) -> i32 {
        match self {
            Operator::Add => operand1 + operand2,
            Operator::Subtract => operand1 - operand2,
            Operator::Multiply => operand1 * operand2,
            Operator::Divide => operand1 / operand2,
            Operator::Unkown => 0,
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let command = command.trim_start_matches("What is ").trim_end_matches('?');
    let command = command.replace("by", "");

    let mut operands: VecDeque<i32> = VecDeque::new();
    let mut operators: VecDeque<Operator> = VecDeque::new();
    let mut previous_was_operand = false;
    let mut notation_mismatch = false;
    command.split_whitespace().for_each(|word| {
        if let Ok(number) = word.parse::<i32>() {
            operands.push_back(number);
            if previous_was_operand {
                notation_mismatch = true;
            }
            previous_was_operand = true;
        } else {
            let operator = match word {
                "plus" => Operator::Add,
                "minus" => Operator::Subtract,
                "multiplied" => Operator::Multiply,
                "divided" => Operator::Divide,
                _ => Operator::Unkown,
            };
            operators.push_back(operator);
            previous_was_operand = false;
        }
    });

    if notation_mismatch {
        return None;
    }

    loop {
        let operator = operators.front().copied();
        let operand1 = operands.front().copied();
        let operand2 = operands.iter().nth(1).copied();
        if let Some(operator) = operator
            && let Some(operand1) = operand1
            && let Some(operand2) = operand2
        {
            operators.pop_front();
            operands.pop_front();
            operands.pop_front();
            operands.push_front(operator.do_operation(operand1, operand2));
        } else {
            break;
        }
    }

    let result = operands.pop_front();

    if operands.is_empty() && operators.is_empty() {
        result
    } else {
        None
    }
}
