use crate::problems::Solution;

impl Solution {
    /// Given two strings `needle` and `haystack`, return the index of the
    /// first occurrence of `needle` in `haystack`, or `-1` if `needle` is not part of `haystack`.
    #[allow(unused)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Iterate through both strings, if there is not a match of char,
        // needle's index resets to 0. The test automatically fails if the rest
        // of needle is longer than the rest of haystack

        if needle.len() > haystack.len() {
            return -1;
        }

        let mut matching_i = -1;
        let mut haystack_i = 0;
        let mut needle_i = 0;

        while haystack_i < haystack.len() && needle_i < needle.len() {
            if haystack.chars().nth(haystack_i) == needle.chars().nth(needle_i) {
                if matching_i == -1 {
                    matching_i = haystack_i as i32;
                }
                haystack_i += 1;
                needle_i += 1;
            } else {
                if matching_i == -1 {
                    haystack_i += 1;
                } else {
                    haystack_i = (matching_i as usize) + 1;
                    matching_i = -1;
                }
                needle_i = 0;
                if (needle.len() - needle_i) > (haystack.len() - haystack_i) {
                    return -1
                }
            }
        }
        matching_i
    }
}

#[cfg(test)]
#[test]
fn test_01() {
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");
    assert_eq!(Solution::str_str(haystack, needle), 0);
}

#[test]
fn test_02() {
    let haystack = String::from("leetcode");
    let needle = String::from("leeto");
    assert_eq!(Solution::str_str(haystack, needle), -1);
}

#[test]
fn test_03() {
    let haystack = String::from("aaa");
    let needle = String::from("aaaa");
    assert_eq!(Solution::str_str(haystack, needle), -1)
}

#[test]
fn test_04() {
    let haystack = String::from("mississippi");
    let needle = String::from("issip");
    assert_eq!(Solution::str_str(haystack, needle), 4);
}
