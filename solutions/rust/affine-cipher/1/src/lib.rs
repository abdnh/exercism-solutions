/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (a.max(b), a.min(b));
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && !c.is_ascii_punctuation())
        .enumerate()
        .map(|(idx, c)| {
            let c = if c.is_alphabetic() {
                let base = 'a' as i32;
                let i = c as i32 - 'a' as i32;
                let e = a * i + b;
                (base + e % 26) as u8 as char
            } else {
                c
            };

            if idx != 0 && (idx + 1) % 5 == 0 {
                format!("{} ", c)
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
        .trim()
        .into())
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn inverse_modulo(a: i32, n: i32) -> Option<i32> {
    let mut t = 0;
    let mut r = n;
    let mut newt = 1;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }
    if r > 1 {
        return None;
    } else if t < 0 {
        t += n;
    }

    Some(t)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let Some(inv_mod) = inverse_modulo(a, 26) else {
        return Err(AffineCipherError::NotCoprime(a));
    };
    Ok(ciphertext
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            if !c.is_alphabetic() {
                return c;
            }
            let base = 'a' as i32;
            let y = (c as u8 - b'a') as i32;
            let d = inv_mod * (y - b);

            (base + d.rem_euclid(26)) as u8 as char
        })
        .collect())
}
