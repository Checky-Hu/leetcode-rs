use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1162261467 % n == 0
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Power of 3: {}", Solution::is_power_of_three(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
