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
        let mut max = {
            let mut val = 0.0;
            if x > 46341 {
                val = 46341.0
            } else {
                val = (x.saturating_add(1)) as f32;
            }
            val
        };
        let mut ptr = max / 2.0;

        let acceptable_deviation = 0.01;

        loop {
            let mut deviation = (ptr * ptr - x as f32);
            if deviation < acceptable_deviation && deviation > -acceptable_deviation {
                break;
            }
            else if (ptr * ptr - x as f32) > acceptable_deviation {
                max = ptr;
            } else if (ptr * ptr - x as f32) < -acceptable_deviation {
                min = ptr;
            }
            ptr = (min + max) / 2.0;
        }
        let next_possible_int = min.floor() as i32 + 1;
        let (possible_value, overflow) = next_possible_int.overflowing_mul(next_possible_int);
        if !overflow && possible_value == x {
            return next_possible_int;
        }

        min.floor() as i32
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

#[test]
fn example_05() {
    let x = 1;
    assert_eq!(Solution::my_sqrt(x), 1);
}

#[test]
fn example_06() {
    let x = 1024;
    assert_eq!(Solution::my_sqrt(x), 32);
}

#[test]
fn example_07() {
    let x = 9;
    assert_eq!(Solution::my_sqrt(x),3);
}

#[test]
fn example_08() {
    let x = 2147395599;
    assert_eq!(Solution::my_sqrt(x), 46339);
}

#[test]
fn example_09() {
    let x = 2;
    assert_eq!(Solution::my_sqrt(x), 1);
}

#[test]
fn example_10() {
    let x = 2147483647;
    assert_eq!(Solution::my_sqrt(x), 46340);
}
