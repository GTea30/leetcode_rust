use crate::problems::Solution;

impl Solution {
    /// Given a string `text`, you want to use the characters of `text` to form as many instances
    /// of the word "balloon" as possible, using each character only a single time.
    #[allow(unused)]
    pub fn max_number_of_balloons(text: String) -> i32 {
        let count = Solution::find_baloon_with_loop(text);
        count as i32
    }

    // Find string "balloon", and remove the letters used
    fn find_baloon_with_loop(text: String) -> u32 {
        let mut count: u32 = 0;
        let mut work_string = text.clone().chars().collect::<Vec<char>>();
        let mut test_string = String::from("balloon").chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut i2 = 0;

        while i < work_string.len() {
            while i2 < test_string.len() {
                if i >= work_string.len() {
                    break;
                }
                if work_string[i] == test_string[i2] {
                    work_string.remove(i);
                    test_string.remove(i2);

                    if test_string.is_empty() {
                        count += 1;
                        test_string = String::from("balloon").chars().collect::<Vec<char>>();
                        i = 0;
                    }

                    i2 = 0;
                    continue;
                }

                i2 += 1;
            }
            i += 1;
            i2 = 0;
        }

        count
    }
}

#[cfg(test)]
#[test]
fn test_01() {
    let input_text = String::from("nlaebolko");
    assert_eq!(Solution::max_number_of_balloons(input_text), 1);
}

#[test]
fn test_02() {
    let input_text = String::from("loonbalxballpoon");
    assert_eq!(Solution::max_number_of_balloons(input_text), 2);
}

#[test]
fn test_03() {
    let input_text = String::from("leetcode");
    assert_eq!(Solution::max_number_of_balloons(input_text), 0);
}
