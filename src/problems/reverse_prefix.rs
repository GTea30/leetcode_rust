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
        // If the char is not in word, return the word
        if !word.contains(ch) {
            return word;
        }

        // Have a pointer start from 0 and move forward until it hits `ch`.
        let mut ch_index = 0;
        while ch_index < word.len() {
            if word.chars().nth(ch_index).expect("Not a char") == ch {
                break;
            }
            ch_index += 1;
        }

        // When it hits `ch`, have the pointer decrement and add each char of
        // word to a new string.
        let mut new_word = String::new();
        for i in (0..ch_index + 1).rev() {
            new_word.push(word.chars().nth(i).expect("Not a char"));
        }

        // Finally, add the end of word to the new string.
        new_word += {
            let (first, last) = word.split_at(ch_index + 1);
            last
        };

        new_word
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
