use std::env;

struct Solution {}

impl Solution {
    fn get_value(s: String) -> i32 {
        let mut result: i32 = 0;
        for c in s.chars() {
            result = result * 10 + c as i32 - 97;
        }
        result
    }

    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        Solution::get_value(first_word) + Solution::get_value(second_word)
            == Solution::get_value(target_word)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut first_word: String = String::new();
    let mut second_word: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => first_word = arg,
            2 => second_word = arg,
            _ => {
                ret += 1;
                let target_word: String = arg;
                println!(
                    "Sum equal: {}",
                    Solution::is_sum_equal(first_word, second_word, target_word)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
