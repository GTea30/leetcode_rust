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
        // Max sqrt of int is 46340
        // Square Root of X is always less than half (unless x <= 4);
        let mut ptr = {
            let mut return_value = 1.0;
            if x <= 1 {
                return_value = x as f32;
            } else {
                return_value = x as f32 / 2.0;

                if return_value > 46340.0 {
                    return_value = 46340.0;
                }
            }
            return_value
        };

        if ptr as i32 * ptr as i32 == x {
            return ptr as i32;
        }

        let mut diff = 1.0;
        let accuracy: f32 = 0.01;
        while diff > accuracy {
            let n = 0.5 * (ptr + (x as f32 / ptr));
            diff = ptr - n;
            ptr = n;
        }

        if ptr as i32 * ptr as i32 > x {
            return ptr as i32 - 1;
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
    assert_eq!(Solution::my_sqrt(x), 3);
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
