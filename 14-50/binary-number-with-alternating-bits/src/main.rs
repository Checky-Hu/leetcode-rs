use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut tmp: i32 = n;
        let mut result: i32 = tmp & 1;
	while tmp > 0 {
	    tmp >>= 1;
	    let flag: i32 = tmp & 1;
	    if result == flag {
	        return false
	    } else {
	        result = flag;
	    }
	}
	true
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
	    ret = 1;
	    let n: i32 = i32::from_str(&arg).expect("Error parse");
            println!("Alternating: {}", Solution::has_alternating_bits(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
