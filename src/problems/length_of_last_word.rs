use crate::problems::Solution;

impl Solution {
    /// Given a string `s` consisting of words and spaces, return the length of
    /// the last word in the string.
    ///
    /// A word is a maximal substring consisting of non-space characters only.
    #[allow(unused)]
    pub fn length_of_last_word(s: String) -> i32 {
        for word in s.split(' ').rev() {
            if !word.is_empty() {
                return word.len() as i32;
            }
        }

        0
    }
}

#[cfg(test)]
#[test]
fn test_01() {
    let s = String::from("Hello World");
    assert_eq!(Solution::length_of_last_word(s), 5);
}

#[test]
fn test_02() {
    let s = String::from("   fly me   to   the moon  ");
    assert_eq!(Solution::length_of_last_word(s), 4);
}

#[test]
fn test_03() {
    let s = String::from("luffy is still joyboy");
    assert_eq!(Solution::length_of_last_word(s), 6);
}
