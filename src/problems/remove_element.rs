use crate::problems::Solution;

impl Solution {
    #[allow(unused)]
    /// Given an integer array of `nums` and an integer `val`, remove all
    /// occurrences of `val` in `nums` in-place. The order of the elements may
    /// be changed. Then return the number of elements in `nums` which are not equal to `val`.
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                nums.remove(i);
            } else {
                i += 1;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
#[test]
fn test_01() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    assert_eq!(Solution::remove_element(&mut nums, val), 2);
    assert_eq!(nums, vec![2, 2]);
}

#[test]
fn test_02() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    assert_eq!(Solution::remove_element(&mut nums, val), 5);
    nums.sort();
    assert_eq!(nums, vec![0, 0, 1, 3, 4]);
}
