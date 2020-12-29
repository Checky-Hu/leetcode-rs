use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut result: i64 = 0;
        for i in 1..=n {
            let mut bit: i64 = 0;
            let mut mut_i: i32 = i;
            while mut_i > 0 {
                mut_i >>= 1;
                bit += 1;
            }
            result = ((result << bit) + i as i64) % 1_000_000_007;
        }
        result as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Concatenated binary: {}", Solution::concatenated_binary(n));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
