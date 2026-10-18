use std::fmt::Write;

struct Run {
    chr: char,
    length: usize,
}

fn encode_run(buffer: &mut String, run: &Run) {
    if run.length > 1 {
        write!(buffer, "{}", run.length).unwrap();
    }
    buffer.write_char(run.chr).unwrap();
}

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();

    let mut current_run: Option<Run> = None;
    for chr in source.chars() {
        if let Some(run) = current_run.as_mut() {
            if run.chr == chr {
                run.length += 1;
            } else {
                encode_run(&mut encoded, run);
                current_run = Some(Run { chr, length: 1 });
            }
        } else {
            current_run = Some(Run { chr, length: 1 });
        }
    }
    if let Some(run) = current_run {
        encode_run(&mut encoded, &run);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();

    let mut chars = source.chars().peekable();
    while let Some(chr) = chars.next() {
        let mut numstr = String::new();
        if chr.is_ascii_digit() {
            numstr.push(chr);
            while let Some(chr2) = chars.peek().copied()
                && chr2.is_ascii_digit()
            {
                chars.next();
                numstr.push(chr2);
            }
            let n: usize = numstr.parse().unwrap();
            if let Some(chr2) = chars.next() {
                decoded.write_str(&chr2.to_string().repeat(n)).unwrap();
            } else {
                // Number followed by nothing - print as is
                decoded.write_str(&numstr).unwrap();
            }
        } else {
            decoded.write_char(chr).unwrap();
        }
    }

    decoded
}
