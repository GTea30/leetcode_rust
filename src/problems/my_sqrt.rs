use super::Solution;

impl Solution {
    /// https://leetcode.com/problems/sqrtx/
    ///
    /// Given a non-negative integer `x`, return the square root of `x` 
    /// rounded down to the nearest integer. The returned integer should be
    /// non-negative as well.
    ///
    /// You must not use any built-in exponent function or operator.
    ///
    /// For example, do not use `pow(x, 0.5)` in c++ or (x ** 0.5) in python.
    #[allow(unused)]
    pub fn my_sqrt(x: i32) -> i32 {
        let mut min = 0.0;
        let mut max = x as f32 - 1.0;
        let mut ptr = (min + max) / 2.0;

        loop {
            let mut deviation = (ptr * ptr - x as f32);
            if deviation < 0.10 && deviation > -0.10 {
                break;
            }
            else if (ptr * ptr - x as f32) > 0.10 {
                max = ptr;
            } else if (ptr * ptr - x as f32) < -0.10 {
                min = ptr;
            }
            ptr = (min + max) / 2.0;
        }

        ptr.floor() as i32
    }
}

#[test]
fn example_01() {
    let x = 4;
    assert_eq!(Solution::my_sqrt(x), 2);
}

#[test]
fn example_02() {
    let x = 8;
    assert_eq!(Solution::my_sqrt(x), 2);
}

#[test]
fn example_03() {
    let x = 2147395600;
    assert_eq!(Solution::my_sqrt(x), 46340);
}

#[test]
fn example_04() {
    let x = 0;
    assert_eq!(Solution::my_sqrt(x), 0);
}
