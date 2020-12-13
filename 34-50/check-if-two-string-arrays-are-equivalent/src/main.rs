use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn array_string_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut s1: String = String::new();
        for word in word1 {
            s1.push_str(&word);
        }
        let mut s2: String = String::new();
        for word in word2 {
            s2.push_str(&word);
        }
        s1 == s2
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut word1: Vec<String> = Vec::new();
    let mut word2: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                if word1.len() < len {
                    word1.push(arg);
                } else {
                    word2.push(arg);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Array string are equal: {}",
        Solution::array_string_are_equal(word1, word2)
    );
}
