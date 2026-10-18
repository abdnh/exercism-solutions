use std::cmp::max;

#[derive(Debug)]
pub struct Item {
    pub value: u32,
    pub weight: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut m: Vec<Vec<u32>> = (0..items.len() + 1)
        .map(|_| (0..max_weight + 1).map(|_| 0).collect::<Vec<u32>>())
        .collect();
    for i in 1..items.len() + 1 {
        for w in 1..max_weight as usize + 1 {
            let item_i = &items[i - 1];
            if item_i.weight > w as u32 {
                m[i][w] = m[i - 1][w];
            } else {
                m[i][w] = max(
                    m[i - 1][w],
                    m[i - 1][w - item_i.weight as usize] + item_i.value,
                );
            }
        }
    }

    m[items.len()][max_weight as usize]
}
