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
        todo!();        
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
