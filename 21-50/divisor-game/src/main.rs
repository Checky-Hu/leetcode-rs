use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n & 1 == 0
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => {
	            ret = 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("First win: {}", Solution::divisor_game(n));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

