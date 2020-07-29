use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut result: i32 = 0;
        let mut tmp: i32 = start;
        for _i in 0..n {
            result ^= tmp;
            tmp += 2;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let start: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "XOR operation in an array: {}",
                    Solution::xor_operation(n, start)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
