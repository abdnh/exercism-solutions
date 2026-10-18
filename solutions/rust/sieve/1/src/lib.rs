use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound)
        .fold(
            (vec![], HashSet::new()),
            |(mut primes, mut marked), elem| {
                if !marked.contains(&elem) {
                    primes.push(elem);
                    marked.extend((elem * 2..=upper_bound).step_by(elem as usize));
                }
                (primes, marked)
            },
        )
        .0
}
