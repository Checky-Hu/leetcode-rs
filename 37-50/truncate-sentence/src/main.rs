use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut result: String = String::new();
        let mut i: i32 = 0;
        for c in s.chars() {
            if c == ' ' {
                if i + 1 == k {
                    break;
                } else {
                    i += 1;
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Truncate sentence: {}", Solution::truncate_sentence(s, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
