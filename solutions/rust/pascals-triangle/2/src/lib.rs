pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut res: Vec<Vec<u32>> = Vec::new();
        for i in 0..self.row_count {
            if let Some(last_line) = res.last() {
                let mut line: Vec<u32> = (0..i)
                    .map(|index| match index {
                        0 => 1,
                        _ if index == i => 1,
                        _ => last_line[index as usize] + last_line[index as usize - 1],
                    })
                    .collect();
                line.push(1);
                res.push(line);
            } else {
                res.push(vec![1]);
            }
        }
        res
    }
}
