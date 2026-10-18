pub fn actions(mut n: u8) -> Vec<&'static str> {
    let mut decoded: Vec<&str> = ["wink", "double blink", "close your eyes", "jump"]
        .iter()
        .copied()
        .filter(|_| {
            let bit = n & 1;
            n >>= 1;
            bit == 1
        })
        .collect();
    if n & 1 == 1 {
        decoded.reverse();
    }

    decoded
}
