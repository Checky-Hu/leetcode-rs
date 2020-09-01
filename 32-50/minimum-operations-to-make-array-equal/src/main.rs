use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        if n & 1 == 1 {
            (n * n - 1) / 4
        } else {
            n * n / 4
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Minimum operations: {}", Solution::min_operations(n));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
