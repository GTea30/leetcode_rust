use crate::problems::Solution;

impl Solution {
    /// Given a sorted array of distinct integers and a target value, return
    /// the index if the target is found. If not, return the index where it
    /// would be if it were inserted in order.
    ///
    /// You must write an algorithm with `0(log n)` runtime complexity
    #[allow(unused)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;

        while i < nums.len() {
            if nums[i] >= target {
                return i as i32;
            }

            i += 1;
        }

        i as i32
    }
}

#[cfg(test)]
#[test]
fn test_01() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(Solution::search_insert(nums, target), 2);
}

#[test]
fn test_02() {
    let nums = vec![1, 3, 5, 6];
    let target = 2;
    assert_eq!(Solution::search_insert(nums, target), 1);
}

#[test]
fn test_03() {
    let nums = vec![1, 3, 5, 6];
    let target = 7;
    assert_eq!(Solution::search_insert(nums, target), 4)
}
