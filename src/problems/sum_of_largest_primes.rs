use super::Solution;

impl Solution {
    /// Given a string `s`, find the sum of the 3 largest unique prime numbers
    /// that can be formed using any of its substrings.
    ///
    /// Return the sum of the three largest unique prime numbers that can be
    /// formed. If fewer than three exist, return the sum of all available
    /// primes. If no prime numbers can be formed, return 0.
    ///
    /// Note: Each prime number should be counted only once, eve if it appears
    /// in multiple substrings. Additionally, when converting a substring to
    /// an integer, any leading zeroes are ignored.
    #[allow(unused)]
    pub fn sum_of_largest_primes(s: String) -> i64 {
        // Find all possible substrings of scrambling s. Add them to a vec if 
        // they're prime.
        //
        // Until the vec is size 3, find all possible substrings of scrambling 
        // s with decrementing length.
        todo!();
    }
}

#[test]
fn example_01() {
    let s = String::from("12234");
    assert_eq!(Solution::sum_of_largest_primes(s), 1469);
}

#[test]
fn example_02() {
    let s = String::from("111");
    assert_eq!(Solution::sum_of_largest_primes(s), 11);
}
