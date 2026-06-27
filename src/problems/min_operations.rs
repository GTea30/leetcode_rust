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
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        // Check if nums is strictly increasing.

        // Find the index of the largest element. For [1, 5, 2, 4, 1], you'd
        // find the value 5 for index 1
        //
        // TODO! When you have found the value, make two subvecs, one from [0...i] another from [i..-1]
        let (index, largest_val) = Solution::find_largest_with_index(&nums);
        //
        // Increment the elements after i until they match i.
        let mut i = index + 1;
        while i < nums.len() {
            if nums[i] < largest_val {
                nums[i] = nums[i] + 1;
                count += 1;
            } else {
                i += 1;
            }
        }
        // Increment each element after i by ni - i times.
        i = index + 1;
        while i < nums.len() {
            for _ in 0..(i - index) {
                nums[i] = nums[i] + 1;
                count += 1;
            }
        }

        return count;
    }

    fn is_strictly_increasing(nums: &Vec<i32>) -> bool {
        let mut i = 0;
        while i < (nums.len() - 1) {
            if nums[i] >= nums[i + 1] {
                return false;
            }

            i += 1;
        }
        return true;
    }

    fn find_largest_with_index(nums: &Vec<i32>) -> (usize, i32) {
        let mut i = 0;
        let mut acc = 0;
        let mut acc_i = 0;

        // while i < len(nums)

        return (acc_i, acc);
    }

    fn recursively_split_by_largest_and_find_operation_count(nums: &Vec<i32>, acc: i32) -> i32 {
        todo!()
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

fn test_03() {
    let nums = vec![8];
    assert_eq!(Solution::min_operations(nums), 0);
}
