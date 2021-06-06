use std::env;

struct Solution {}

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let mut status: (char, i32) = ('0', 1);
        let mut result: i32 = 0;
        for c in s.chars() {
            if c == status.0 {
                status.1 += 1;
            } else {
                status.0 = c;
                status.1 = 1;
            }
            result = (result + status.1) % modulo;
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
                    "Number of homogenous substrings: {}",
                    Solution::count_homogenous(s)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
