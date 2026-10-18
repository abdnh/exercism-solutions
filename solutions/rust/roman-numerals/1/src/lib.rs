use std::{
    fmt::{Display, Formatter, Result},
    unreachable,
};

pub struct Roman {
    value: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.value)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mut value: Vec<&str> = vec![];

        let mut digits: u32 = 0;
        while num != 0 {
            let d = num % 10;
            num /= 10;
            digits += 1;
            // The Big Match
            let numerals = match d {
                0 => "",

                1 if digits == 1 => "I",
                1 if digits == 2 => "X",
                1 if digits == 3 => "C",
                1 => "M",

                2 if digits == 1 => "II",
                2 if digits == 2 => "XX",
                2 if digits == 3 => "CC",
                2 => "MM",

                3 if digits == 1 => "III",
                3 if digits == 2 => "XXX",
                3 if digits == 3 => "CCC",
                3 => "MMM",

                4 if digits == 1 => "IV",
                4 if digits == 2 => "XL",
                4 if digits == 3 => "CD",

                5 if digits == 1 => "V",
                5 if digits == 2 => "L",
                5 if digits == 3 => "D",

                6 if digits == 1 => "VI",
                6 if digits == 2 => "LX",
                6 if digits == 3 => "DC",

                7 if digits == 1 => "VII",
                7 if digits == 2 => "LXX",
                7 if digits == 3 => "DCC",

                8 if digits == 1 => "VIII",
                8 if digits == 2 => "LXXX",
                8 if digits == 3 => "DCCC",

                9 if digits == 1 => "IX",
                9 if digits == 2 => "XC",
                9 if digits == 3 => "CM",

                _ => unreachable!(),
            };
            value.push(numerals);
        }
        value.reverse();

        Self {
            value: value.join(""),
        }
    }
}
