use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if k & 1 == 1 {
            if Solution::kth_grammar(n - 1, (k + 1) / 2) == 1 {
                1
            } else {
                0
            }
        } else {
            if Solution::kth_grammar(n - 1, k / 2) == 1 {
                0
            } else {
                1
            }
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            n = i32::from_str(&arg).expect("Error parse.");
        } else if 2 == index {
            let k: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Grammar: {}", Solution::kth_grammar(n, k));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
