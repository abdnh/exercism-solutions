pub fn raindrops(n: u32) -> String {
    let mut words = vec![];
    if n % 3 == 0 {
        words.push("Pling");
    }
    if n % 5 == 0 {
        words.push("Plang");
    }
    if n % 7 == 0 {
        words.push("Plong");
    }

    if words.is_empty() {
        n.to_string()
    } else {
        words.join("")
    }
}
