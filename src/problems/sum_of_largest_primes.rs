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
        let mut prime_vec: Vec<i64> = Vec::new();

        if Solution::is_prime(s.parse::<i64>().expect("Not a number")) {
            prime_vec.push(s.parse::<i64>().expect("Not a number"));
        }

        let mut l = s.len() - 1;
        // While l is >= 0, move the slice around (`&s[start..start + n]`).
        // If prime_vec is ever greater than 3, break;

        prime_vec.iter().sum()
    }

    fn is_prime(n: i64) -> bool {
        if n % 2 == 0 {
            return false;
        }
        let mut i = 3;
        while i < n.isqrt() + 1 {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }

        true
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
