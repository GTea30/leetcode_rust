use super::Solution;

impl Solution {
    /// https://leetcode.com/problems/plus-one/
    ///
    /// You are given a large integer represented as an integer array
    /// `digits`, where each `digits[i]` is the `ith` digit of the integer.
    /// The digits are ordered from most significant to least significant in
    /// left-to-right order. The large integer does not contain any leading
    /// `0`'s.
    ///
    /// Increment the large integer by one and return the resulting array of
    /// digits.
    #[allow(unused)]
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        todo!();
    }
}

#[test]
fn example_01() {
    let digits = vec![1, 2, 3];
    assert_eq!(Solution::plus_one(digits), vec![1, 2, 4]);
}

#[test]
fn example_02() {
    let digits = vec![4, 3, 2, 1];
    assert_eq!(Solution::plus_one(digits), vec![4, 3, 2, 2]);
}

#[test]
fn example_03() {
    let digits = vec![9];
    assert_eq!(Solution::plus_one(digits), vec![1, 0]);
}
