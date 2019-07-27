use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut result: i32 = 0;
	let mut pre_1: i32 = 0;
	let mut i: i32 = 1;
	let mut tmp: i32 = n;
	while tmp > 0 {
	    if 1 == tmp & 1 {
	        if pre_1 != 0 && i - pre_1 > result {
		    result = i - pre_1;
		}
		pre_1 = i;
	    }
	    i += 1;
	    tmp >>= 1;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    println!("Distance: {}", Solution::binary_gap(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

