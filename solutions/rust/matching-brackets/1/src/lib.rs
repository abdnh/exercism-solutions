fn is_opening_bracket(c: char) -> bool {
    matches!(c, '[' | '{' | '(')
}

fn closing_bracket_for_opening(c: char) -> Option<char> {
    Some(match c {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        _ => return None,
    })
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];
    for c in string.chars() {
        match c {
            '[' | '{' | '(' | ']' | '}' | ')' => brackets.push(c),
            _ => continue,
        }
    }
    let mut closing = vec![];
    while !brackets.is_empty() {
        let b = brackets.pop().unwrap();
        if !is_opening_bracket(b) {
            closing.push(b);
        } else if closing.is_empty() {
            return false;
        } else {
            let c = closing.pop().unwrap();
            if c != closing_bracket_for_opening(b).unwrap() {
                return false;
            }
        }
    }

    closing.is_empty()
}
