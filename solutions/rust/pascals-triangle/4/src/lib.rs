pub struct PascalsTriangle {
    row_count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count: row_count as usize,
        }
    }
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut res: Vec<Vec<u32>> = Vec::with_capacity(self.row_count);
        for row_num in 0..self.row_count {
            let mut row = vec![1; row_num + 1];
            for index in 1..row_num {
                let prev_row = &res[row_num - 1];
                row[index] = prev_row[index - 1] + prev_row[index];
            }
            res.push(row);
        }
        res
    }
}
