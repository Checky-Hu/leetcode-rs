use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut result: i32 = 0;
	let mut tmp: i32 = n;
	while tmp > 0 {
	    tmp /= 5;
	    result += tmp;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Zeroes: {}", Solution::trailing_zeroes(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
