pub struct PascalsTriangle {
    lines: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let pascals = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
            vec![1, 7, 21, 35, 35, 21, 7, 1],
            vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
        ];
        Self {
            lines: match row_count {
                0 => vec![],
                i if i <= 10 => pascals.into_iter().take(i as usize).collect(),
                _ => unreachable!(),
            },
        }
    }
    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.lines.clone()
    }
}