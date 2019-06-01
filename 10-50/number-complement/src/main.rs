use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
	let mut tmp: i32 = num;
	let mut cur: i32 = 1;
	let mut result: i32 = 0;
	while tmp > 0 {
	    if tmp & 1 == 0 {
	        result += cur;
	    }
	    cur <<= 1;
	    tmp >>= 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Complement: {}", Solution::find_complement(num));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
