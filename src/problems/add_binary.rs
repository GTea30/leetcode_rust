use super::Solution;

impl Solution {
    /// Given two binary strings `a` and `b`, return their sum as a binary string
    #[allow(unused)]
    pub fn add_binary(a: String, b: String) -> String {
        // Have two pointers pointing at the end of both strings.
        let mut a_ptr = a.len() - 1;
        let mut b_ptr = b.len() - 1;
        let mut bank = 0;
        let mut a_finished_flag = false;
        let mut b_finished_flag = false;

        let mut new_string = String::new();

        let mut finished_flag = false;

        while !a_finished_flag || !b_finished_flag  || bank > 0{
            // Addition
            bank += Solution::index_string(&a, a_ptr, a_finished_flag) + Solution::index_string(&b, b_ptr, b_finished_flag);
            if bank % 2 == 0 {
                new_string.insert(0, '0');
            } 
            else {
                new_string.insert(0, '1');
            }
            bank = bank.saturating_sub(1);

            if a_ptr == 0 {
                a_finished_flag = true;
            }
            if b_ptr == 0 {
                b_finished_flag = true;
            }
            a_ptr = a_ptr.saturating_sub(1);
            b_ptr = b_ptr.saturating_sub(1);
        }

        // If one of the pointers has a relative address of `0`, let the value = 0;
        new_string
    }

    fn index_string(s: &str, i: usize, finished_flag: bool) -> u32 {
        if finished_flag {
            return 0
        }
        if s.chars().nth(i).expect("Not a character") == '0' {
            0
        } else {
        1
    }
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
