/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut distance = 0;
    let chars1 = s1.chars();
    let mut chars2 = s2.chars();
    for c1 in chars1 {
        let c2 = chars2.next().unwrap();
        if c1 != c2 {
            distance += 1;
        }
    }

    Some(distance)
}
