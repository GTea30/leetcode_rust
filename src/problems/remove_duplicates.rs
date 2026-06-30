use crate::problems::Solution;

impl Solution {
    /// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
    /// Given an integer array `nums` sorted in non-decreasing order, remove the duplicates in-place
    /// such that each unique element only appears once. The relative order of the elements should
    /// be kept the same.
    ///
    /// Consider the number of unique elements in `nums` to be `k`. After removing duplicates,
    /// return the number of unique elements `k`.
    ///
    /// The first `k` elements of `nums` should contain the unique numbers in sorted order. The
    /// remaining elements beyond index `k-1` can be ignored
    #[allow(unused)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut duplicates: Vec<i32> = vec![];
        let mut i = 0;
        while i < nums.len() {
            if duplicates.contains(&nums[i]) {
                nums.remove(i);
            } else {
                duplicates.push(nums[i]);
                i += 1;
            }
        }

        nums.len() as i32
    }
}

#[cfg(test)]
#[test]
fn test_01() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    assert_eq!(nums, vec![1, 2]);
}

#[test]
fn test_02() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    assert_eq!(nums, vec![0, 1, 2, 3, 4])
}
