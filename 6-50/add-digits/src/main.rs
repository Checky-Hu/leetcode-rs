use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
	    0
	} else {
	    (num - 1) % 9 + 1
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Digit: {}", Solution::add_digits(num));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
