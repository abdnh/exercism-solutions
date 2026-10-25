static DISCOUNTED_PRICE: [u32; 5] = [800, 760, 720, 640, 600];

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts = vec![0; 5];
    for book in books {
        if *book <= 5 {
            counts[*book as usize - 1] += 1;
        }
    }
    counts.sort_by(|a, b| b.cmp(a));

    (1..=5)
        .map(|mut max_group| {
            let mut sum = 0;
            let mut copy = counts.clone();
            while copy.iter().any(|e| *e != 0) {
                let mut count = 0;
                for item in &mut copy {
                    if count < max_group && *item != 0 {
                        *item -= 1;
                        count += 1;
                    }
                }
                if count != 0 {
                    sum += DISCOUNTED_PRICE[count as usize - 1] * count;
                }
                if count < max_group {
                    max_group -= 1;
                }
            }
            sum
        })
        .min()
        .unwrap_or_default()
}
