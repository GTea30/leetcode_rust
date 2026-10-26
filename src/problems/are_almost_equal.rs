use crate::problems::Solution;

impl Solution {
    /// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
    ///
    /// You are given two strings `s1` and `s2` of equal length.
    /// A **string swap** is an operation where you choose two indices in a string
    /// (not necessarily different) and swap the characters at these indices.
    ///
    /// Return `true`
    /// *if it is possible to make both strings equal by performing **at most one string swap** on **exactly one** of the strings.*
    /// Otherwise return `false`
    #[allow(unused)]
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        // Return true if the strings are the same;
        if s1 == s2 {
            return true;
        }
        // Generally speaking: For s1: swap s1[n] and s1[n + 1:-1]. Check equality at every swap.
        // How do we swap?
        for n in 0..(s1.len() - 1) {
            let result: bool = Solution::string_swap(&s1, n, &s2);
            if result {
                return true;
            }
        }
        false
    }

    /// swaps s1[n] with every item s1[n+1..] and checks equality
    fn string_swap(s1: &str, n: usize, s2: &str) -> bool {
        let mut i = n + 1;
        let n_char: char = s1.chars().nth(n).expect("Not a character");
        while i < s1.len() {
            let i_char: char = s1.chars().nth(i).expect("Not a character");
            let new_string: String = s1
                .chars()
                .enumerate()
                .map(|(index, value)| {
                    if index == n {
                        return i_char;
                    } else if index == i {
                        return n_char;
                    }
                    value
                })
                .collect();

            if new_string == s2 {
                return true;
            }

            i += 1;
        }
        false
    }
}

#[test]
fn example_01() {
    let s1: String = String::from("bank");
    let s2: String = String::from("kanb");
    assert!(Solution::are_almost_equal(s1, s2));
}

#[test]
fn example_02() {
    let s1: String = String::from("attack");
    let s2: String = String::from("defend");
    assert!(!Solution::are_almost_equal(s1, s2));
}

#[test]
fn example_03() {
    let s1: String = String::from("kelb");
    let s2: String = String::from("kelb");
    assert!(Solution::are_almost_equal(s1, s2));
}
