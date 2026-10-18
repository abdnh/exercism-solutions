pub fn nth(n: u32) -> u32 {
    // brute force
    let mut count: u32 = 0;
    for i in 2.. {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            count += 1;
        }
        if count - 1 == n {
            return i;
        }
    }

    0
}
