pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }
    if size == 1{
        return vec![vec![1]];
    }

    let size = size as usize;
    let mut matrix = vec![vec![0_u32; size]; size];

    let (mut top, mut bottom, mut left, mut right) = (0, size - 1, 0, size - 1);
    let mut num = 1;

    while top <= bottom && left <= right {
        for col in left..right + 1 {
            matrix[top][col] = num;
            num += 1;
        }
        top += 1;

        for row in top..bottom + 1 {
            matrix[row][right] = num;
            num += 1;
        }
        right -= 1;

        for col in (left..right + 1).rev() {
            matrix[bottom][col] = num;
            num += 1;
        }
        bottom -= 1;

        for row in (top..bottom + 1).rev() {
            matrix[row][left] = num;
            num += 1;
        }
        left += 1;
    }

    matrix
}
