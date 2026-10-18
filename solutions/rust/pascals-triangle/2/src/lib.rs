pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0).fold(vec![], |mut triangle, i| {
            let row = match i {
                0 => vec![1],
                1 => vec![1, 1],
                _ => {
                    let mut row: Vec<u32> = vec![1];
                    let previous_row = triangle.last().unwrap();
                    for i in 0..previous_row.len() {
                        let v1 = previous_row[i];
                        let v2 = previous_row.get(i + 1).copied().unwrap_or_default();
                        row.push(v1 + v2);
                    }

                    row
                }
            };
            triangle.push(row);

            triangle
        })
    }
}
