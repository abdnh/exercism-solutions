pub fn egg_count(mut value: u32) -> usize {
    let mut count = 0;
    while value != 0 {
        if value & 1 == 1 {
            count += 1;
        }
        value >>= 1;
    }

    count
}
