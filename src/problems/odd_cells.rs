use super::Solution;

impl Solution {
    /// There is an `m x n` matrix that is initialized to all `0`'s. There is
    /// also a 2D array `indices` where each `indices[i] = [ri, ci]` represents
    /// a 0-indexed location to perform some increment operations on the
    /// matrix.
    ///
    /// For each location `indices[i]`, do both the following:
    /// - Increment all the cells on row `ri`.
    /// - Increment all the cells on column `ci`.
    ///
    /// Given `m`, `n`, and `indices`, return the number of odd-valued cells
    /// in the matrix after applying the increment to all locations in
    /// `indices`.
    #[allow(unused)]
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        // Initialize matrix as an array
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        for columns in (0..m + 1) {
            matrix.push(vec![0;n as usize]);
        }

        // for each vector in indices, increment a row or column n number of times
        for i in indices {
            Solution::increment_rows(&mut matrix, i[0]);
            Solution::increment_columns(&mut matrix, i[1]);
        }
        // Count odd cells
        let mut count = 0;
        for y in &matrix {
            for x in y {
                if x % 2 != 0 {
                    count += 1;
                }
            }
        }

        count
    }

    fn increment_rows(matrix: &mut [Vec<i32>], r: i32) {
        let mut ptr = 0;
        while ptr < matrix[r as usize].len() {
            matrix[r as usize][ptr] += 1;
            ptr += 1;
        }
    }

    fn increment_columns(matrix: &mut [Vec<i32>], c: i32) {
        let mut ptr = 0;
        while ptr < matrix.len() {
            matrix[ptr][c as usize] += 1;
            ptr += 1;
        }
    }
}

#[test]
fn example_01() {
    let m = 2;
    let n = 3;
    let indices = vec![vec![0, 1], vec![1, 1]];

    assert_eq!(Solution::odd_cells(m, n, indices), 6);
}

#[test]
fn example_02() {
    let m = 2;
    let n = 2;
    let indices = vec![vec![1, 1], vec![0, 0]];

    assert_eq!(Solution::odd_cells(m, n, indices), 6);
}
