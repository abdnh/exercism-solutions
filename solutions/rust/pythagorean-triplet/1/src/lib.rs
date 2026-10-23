use std::collections::HashSet;

fn euclid(m: u32, n: u32, k: u32) -> [u32; 3] {
    let m2 = m.pow(2);
    let n2 = n.pow(2);
    let a = k * (m2 - n2);
    let b = k * 2 * m * n;
    let c = k * (m2 + n2);
    let mut triple = [a, b, c];
    triple.sort();

    triple
}

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for (n, ms) in (1u32..).map(|i| (i, (i + 1..))) {
        let mut ms = ms.peekable();
        let m = ms.peek().copied().unwrap();
        // a + b + c > sum
        if 2 * (m.pow(2) + m * n) > sum {
            break;
        }

        'ms: for m in ms {
            let triple = euclid(m, n, 1);
            let s = triple.iter().sum::<u32>();
            if s == sum {
                triplets.insert(triple);
            } else if s > sum {
                break;
            }
            for k in 2.. {
                let triple = euclid(m, n, k);
                let s = triple.iter().sum::<u32>();
                if s == sum {
                    triplets.insert(triple);
                } else if s > sum {
                    continue 'ms;
                }
            }
        }
    }

    triplets
}
