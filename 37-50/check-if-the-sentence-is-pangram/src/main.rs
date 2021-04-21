use std::env;

struct Solution {}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut flags: Vec<bool> = vec![false; 26];
        let mut count: i32 = 0;
        for c in sentence.chars() {
            let i: usize = c as usize - 97;
            if !flags[i] {
                flags[i] = true;
                count += 1;
            }
        }
        count == 26
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let sentence: String = arg;
                println!("Check if pangram: {}", Solution::check_if_pangram(sentence));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
