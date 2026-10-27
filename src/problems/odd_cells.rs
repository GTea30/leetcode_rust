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
        // Count odd cells
        todo!();
    }
}

#[test]
fn example_01() {
    let m = 2;
    let n = 3;
    let indices = vec![vec![0, 1], vec![1, 1]];

    assert_eq!(Solution::odd_cells(m, n, indices), 6);
}

fn example_02() {
    let m = 2;
    let n = 2;
    let indices = vec![vec![1, 1], vec![0, 0]];

    assert_eq!(Solution::odd_cells(m, n, indices), 6);
}
