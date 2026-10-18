pub struct Matrix {
    data: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let data: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|t| t.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        Self { data }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.data.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let row_len = self.data[0].len();
        if col_no > row_len {
            None
        } else {
            Some(
                self.data
                    .iter()
                    .filter(|row| col_no - 1 < row.len())
                    .map(|row| row[col_no - 1])
                    .collect(),
            )
        }
    }
}
