use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut result: i32 = n;
        while m < result {
            result &= result - 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut m: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => m = i32::from_str(&arg).expect("Error parse."),
            2 => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Result: {}", Solution::range_bitwise_and(m, n));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
