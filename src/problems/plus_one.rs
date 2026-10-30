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
        // Increemnt the last item in the vec.
        //
        // If the incrementved value would equal 10, reset to 0 and increment
        // the next digit.
        let mut new_digits: Vec<i32> = vec![0; digits.len()];

        let mut carry_flag = false;
        let mut incrementing = true;

        for ptr in (0..digits.len()).rev() {
            if !incrementing {
                new_digits[ptr] = digits[ptr];
                continue;
            }

            if digits[ptr] < 9 {
                new_digits[ptr] = digits[ptr] + 1;
                incrementing = false;
            } else if ptr == 0 {
                new_digits.insert(0, 1);
            }
        }

        new_digits
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
