use std::env;

struct Solution {}

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let vec: Vec<&str> = sentence.split(' ').collect();
        let mut result: i32 = -1;
        for (i, v) in vec.iter().enumerate() {
            if v.starts_with(&search_word) {
                result = i as i32 + 1;
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut sentence: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => sentence = arg,
            _ => {
                ret += 1;
                let search_word: String = arg;
                println!(
                    "Prefix of word: {}",
                    Solution::is_prefix_of_word(sentence, search_word)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
