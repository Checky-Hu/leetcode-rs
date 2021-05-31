use std::env;

struct Solution {}

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut status: (char, char) = ('0', '0');
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                status.0 = c;
            } else if i == 1 {
                status.1 = c;
            } else {
                if status.0 != status.1 && c != status.0 && c != status.1 {
                    result += 1;
                }
                status.0 = status.1;
                status.1 = c;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                println!(
                    "Count good substrings: {}",
                    Solution::count_good_substrings(s)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
