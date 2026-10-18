static ONES: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
static TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
static HUNDREDS: [&str; 7] = [
    "quintillion",
    "quadrillion",
    "trillion",
    "billion",
    "million",
    "thousand",
    "",
];

pub fn encode(n: u64) -> String {
    let mut words: Vec<String> = vec![];
    for (i, zeros) in (0..=(HUNDREDS.len() - 1) * 3).rev().step_by(3).enumerate() {
        let d = (n / 10u64.pow(zeros as u32)) % 1000;
        if d == 0 {
            continue;
        }
        let hundreds = d / 100;
        let tens = d % 100;
        if hundreds != 0 {
            words.push(ONES[hundreds as usize].into());
            words.push("hundred".into());
        }
        if (tens as usize) < ONES.len() {
            words.push(ONES[tens as usize].into());
        } else {
            let compound = if tens % 10 == 0 {
                TENS[(tens / 10) as usize].into()
            } else {
                format!(
                    "{}-{}",
                    TENS[(tens / 10) as usize],
                    ONES[(tens % 10) as usize]
                )
            };
            words.push(compound);
        }
        words.push(HUNDREDS[i].into());
    }

    let result = words.join(" ").trim().to_string();

    if result.is_empty() {
        "zero".into()
    } else {
        result
    }
}
