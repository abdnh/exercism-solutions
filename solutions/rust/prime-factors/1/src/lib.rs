pub fn nth_prime(n: u64) -> u64 {
    // brute force
    let mut count: u64 = 0;
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


pub fn factors(n: u64) -> Vec<u64> {
    let mut factors_vec = vec![];
    let mut remainder = n;
    let mut i = 0;
    loop {
        let p = nth_prime(i);
        if remainder % p == 0 {
            factors_vec.push(p);
            remainder /= p;
        } else {
            i += 1;
        }
        if remainder < 2 {
            break;
        }
    }
    factors_vec
}
