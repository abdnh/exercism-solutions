pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut triangle: Vec<Vec<u32>> = vec![];
        if self.0 > 0 {
            triangle.push(vec![1]);
        }
        if self.0 > 1 {
            triangle.push(vec![1, 1]);
        }
        for _ in 2..self.0 {
            let mut row: Vec<u32> = vec![1];
            let previous_row = triangle.last().unwrap();
            for i in 0..previous_row.len() {
                let v1 = previous_row[i];
                let v2 = previous_row.get(i + 1).copied().unwrap_or_default();
                row.push(v1 + v2);
            }
            triangle.push(row);
        }

        triangle
    }
}
