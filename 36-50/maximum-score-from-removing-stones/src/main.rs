use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let max = if a > b {
            if a > c {
                a
            } else {
                c
            }
        } else if b > c {
            b
        } else {
            c
        };
        let sum: i32 = a + b + c;
        if max << 1 > sum {
            sum - max
        } else {
            sum >> 1
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a = i32::from_str(&arg).expect("Error parse."),
            2 => b = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Maximum score from removing stones: {}",
                    Solution::maximum_score(a, b, c)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
