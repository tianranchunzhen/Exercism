pub struct Matrix(Vec<Vec<u32>>);

impl Matrix {
    pub fn new(input: &str) -> Self {
        Matrix(
            input
                .lines()
                .map(|line| line.split(" ").map(|n| n.parse::<u32>().unwrap()).collect())
                .collect(),
        )
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.0.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.0
            .iter()
            .map(|row| row.get(col_no - 1).cloned())
            .collect()
    }
}