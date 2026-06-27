use std::vec;

use crate::problems::Solution;

impl Solution {
    /// You are given an integer array `nums (0-indexed)`. In one operation,
    /// you can choose an element of the array and increment it by 1.
    ///
    /// - For example if `nums = [1, 2, 3]`, you can choose to increment
    /// `nums[1]` to make `nums[1,3,3]`.
    ///
    /// Return the *minimum* number of operations to make `nums` strictly
    /// increasing.
    ///
    /// An array `nums` is strictly increasion if `nums[i] < nums[i+1]`` for all
    /// `0 <= i < nums.length - 1`. An array of length 1 is trivially strictly
    /// increasing
    #[allow(unused)]
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let (inc_vec, sum) = Solution::recursivley_split_and_merge(&nums);
        return sum;
    }

    fn is_strictly_increasing(nums: &Vec<i32>) -> bool {
        if nums.len() == 0 {
            return true;
        }
        let mut i = 0;
        while i < (nums.len() - 1) {
            if nums[i] >= nums[i + 1] {
                return false;
            }

            i += 1;
        }
        return true;
    }

    fn recursivley_split_and_merge(nums: &Vec<i32>) -> (Vec<i32>, i32) {
        // Checks to see if the vec is len <= 1 or otherwise strictly increasing
        if nums.len() <= 1 {
            return (nums.clone(), 0);
        }

        if Solution::is_strictly_increasing(nums) {
            return (nums.clone(), 0);
        }

        // Checks if the first element is the largest, if not, split;
        let highest_index = Solution::find_index_of_highest_value(nums);
        if highest_index == 0 {
            // Runs increment
            return Solution::increment(nums);
        } else {
            // Splits
            let (vec_a, vec_b) = Solution::split_at(nums, highest_index);
            let (mut parsed_vec_a, a_acc) = Solution::recursivley_split_and_merge(&vec_a);
            let (mut parsed_vec_b, b_acc) = Solution::recursivley_split_and_merge(&vec_b);

            // Merges
            parsed_vec_a.append(&mut parsed_vec_b);
            let (parsed_merged_vec, merged_acc) =
                Solution::recursivley_split_and_merge(&parsed_vec_a);
            return (parsed_merged_vec, a_acc + b_acc + merged_acc);
        }
    }

    /// Assumes that the largest number is at the start of the vec
    fn increment(nums: &Vec<i32>) -> (Vec<i32>, i32) {
        let mut count = 0;
        let mut new_vec = vec![nums[0]];
        let mut i = 1;

        // Increments all other numbers to match the 0 index
        while i < nums.len() {
            new_vec.push(nums[i]);
            while new_vec[i] > nums[0] {
                new_vec[i] += 1;
                count += 1;
            }

            i += 1;
        }

        i = 1;

        while i < new_vec.len() {
            new_vec[i] += i as i32;
            count += i as i32;

            i += 1;
        }

        return (new_vec, count);
    }

    fn split_at(nums: &Vec<i32>, split_at: usize) -> (Vec<i32>, Vec<i32>) {
        let mut new_vec_a = vec![];
        let mut new_vec_b = vec![];

        let mut i = 0;
        while i < nums.len() {
            if i < split_at {
                new_vec_a.push(nums[i])
            } else {
                new_vec_b.push(nums[i])
            }
            i += 1;
        }

        return (new_vec_a, new_vec_b);
    }

    fn find_index_of_highest_value(nums: &Vec<i32>) -> usize {
        let mut i = 0;
        let mut acc = 0;
        let mut acc_i = 0;

        while i < nums.len() {
            if nums[i] > acc {
                acc = nums[i];
                acc_i = i
            }
            i += 1;
        }
        return acc_i;
    }
}

#[cfg(test)]
#[test]
fn test_01() {
    let nums = vec![1, 1, 1];
    assert_eq!(Solution::min_operations(nums), 3);
}

#[test]
fn test_02() {
    let nums = vec![1, 5, 2, 4, 1];
    assert_eq!(Solution::min_operations(nums), 14);
}

#[test]
fn test_03() {
    let nums = vec![8];
    assert_eq!(Solution::min_operations(nums), 0);
}

#[test]
fn test_04() {
    let nums = vec![7, 4, 2, 8, 1, 7, 7, 10];
    assert_eq!(Solution::min_operations(nums), 38);
}
