use std::fmt::Write;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags {
    line_number: bool,
    only_filename: bool,
    case_insensitive: bool,
    inverted: bool,
    whole_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut out = Self {
            line_number: false,
            only_filename: false,
            case_insensitive: false,
            inverted: false,
            whole_line: false,
        };
        for flag in flags {
            match *flag {
                "-n" => out.line_number = true,
                "-l" => out.only_filename = true,
                "-i" => out.case_insensitive = true,
                "-v" => out.inverted = true,
                "-x" => out.whole_line = true,
                _ => {}
            }
        }

        out
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let print_filename = files.len() > 1;
    let mut lines: Vec<String> = vec![];
    for file in files.iter().copied() {
        let reader = BufReader::new(File::open(file)?);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let mut matched = if flags.whole_line {
                if flags.case_insensitive {
                    line.to_lowercase() == pattern.to_lowercase()
                } else {
                    line == pattern
                }
            } else if flags.case_insensitive {
                line.to_lowercase().contains(&pattern.to_lowercase())
            } else {
                line.contains(pattern)
            };

            if flags.inverted {
                matched = !matched;
            }
            if !matched {
                continue;
            }

            if flags.only_filename {
                lines.push(file.into());
                break;
            } else {
                let mut buf = String::new();
                if print_filename {
                    write!(buf, "{}:", file)?;
                }
                if flags.line_number {
                    write!(buf, "{}:", i + 1)?;
                }
                write!(buf, "{}", line)?;
                lines.push(buf);
            }
        }
    }

    Ok(lines)
}
