pub fn encrypt(input: &str) -> String {
    let input = input
        .replace(|c: char| !c.is_alphanumeric(), "")
        .to_ascii_lowercase();
    dbg!(&input, input.len());
    let input = input.as_bytes();

    let mut c: usize = 0;
    let mut r: usize = 0;
    'outer: for ci in 1..=input.len() {
        for ri in 1..=input.len() {
            if ci * ri >= input.len() && ci >= ri && ci.saturating_sub(ri) <= 1 {
                c = ci;
                r = ri;
                break 'outer;
            }
        }
    }

    if r == 0 {
        return "".into();
    }

    let v: Vec<u8> = (0..c)
        .flat_map(|ci| (0..r).map(move |ri| input.get(ci + ri * c).copied()))
        .collect::<Vec<_>>()
        .chunks(r)
        .flat_map(|chunk| {
            let mut v: Vec<_> = chunk.iter().map(|c| c.unwrap_or(b' ')).collect();
            v.push(b' ');
            v
        })
        .collect();

    let output = String::from_utf8_lossy(v.as_slice());

    output[0..output.len() - 1].into()
}
