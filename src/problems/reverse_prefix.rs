use super::Solution;

impl Solution {
    /// Given a 0-indexed string `word` and a character `ch`, reverse the 
    /// segment of `word` that starts at index `0` and ends at the index of the
    /// first occurence of `ch`(inclusive). If the character `ch` does not
    /// exist in `word`, do nothing.
    ///
    /// - For example, if `word = "abcdefd"` and `ch = "d"`, then you should
    ///   reverse the segment that starts at `0` and ends at `3`(inclusive).
    ///   The resulting string will be `"dcbaefd"`.
    #[allow(unused)]
    pub fn reverse_prefix(word: String, ch: char) -> String {
        todo!();        
    }
}

#[test]
fn example_01() {
    let word = String::from("abcdefd");
    let ch = 'd';

    assert_eq!(Solution::reverse_prefix(word, ch), String::from("dcbaefd"));
}

#[test]
fn example_02() {
    let word = String::from("xyxzxe");
    let ch = 'z';

    assert_eq!(Solution::reverse_prefix(word, ch), String::from("zxyxxe"));
}

#[test]
fn example_03() {
    let word = String::from("abcd");
    let ch = 'z';

    assert_eq!(Solution::reverse_prefix(word, ch), String::from("abcd"));
}
