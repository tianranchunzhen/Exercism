pub struct Matrix {
    rows: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        Matrix { rows }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.rows
            .iter()
            .map(|row| row.get(col_no - 1).cloned())
            .collect()
    }
}