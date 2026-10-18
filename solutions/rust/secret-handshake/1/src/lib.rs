pub fn actions(mut n: u8) -> Vec<&'static str> {
    let mut decoded = vec![];
    for action in ["wink", "double blink", "close your eyes", "jump"] {
        let bit = n & 1;
        n >>= 1;
        if bit == 1 {
            decoded.push(action);
        }
    }
    if n & 1 == 1 {
        decoded.reverse();
    }

    decoded
}
