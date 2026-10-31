use super::Solution;

impl Solution {
    /// Given two binary strings `a` and `b`, return their sum as a binary string
    #[allow(unused)]
    pub fn add_binary(a: String, b: String) -> String {
        todo!();
    }
}

#[test]
fn example_01() {
    let a = String::from("11");
    let b = String::from("1");
    assert_eq!(Solution::add_binary(a, b), String::from("100"));
}

#[test]
fn example_02() {
    let a = String::from("1010");
    let b = String::from("1011");
    assert_eq!(Solution::add_binary(a, b), String::from("10101"));
}
