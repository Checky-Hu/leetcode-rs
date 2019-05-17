use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut base: i32 = 1;
	let mut result: i32 = 1;
	while base * 2 <= n {
	    result += base;
	    base *= 2;
	    if base * 2 > n {
	        break;
	    } else {
	        if (n / base) % 2 == 1 {
		    result += base;
		}
		base *= 2;
	    }
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
            println!("Remaining: {}", Solution::last_remaining(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
