use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        if num <= 0 || num & (num - 1) != 0 {
            return false
        }
        num & 0x55555555 != 0
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Power of 4: {}", Solution::is_power_of_four(num));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
