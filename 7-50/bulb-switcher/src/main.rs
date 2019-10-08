use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn bulb_switcher(n: i32) -> i32 {
        let mut result: i32 = 1;
        while result * result <= n {
            result += 1;
        }
        result - 1
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Bulb lights on: {}", Solution::bulb_switcher(n));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
