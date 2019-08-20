use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1
        }
        let mut bit: i32 = 0;
        let mut result: i32 = 0;
        let mut tmp: i32 = n;
        while tmp > 0 {
            if tmp & 1 == 0 {
                result += 1 << bit;
            }
            bit += 1;
            tmp >>= 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => {
	            ret = 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Complement: {}", Solution::bitwise_complement(n));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

