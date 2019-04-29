use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num <= 0 {
	    false
	} else {
	    let mut tmp: i32 = num;
	    while tmp != 1 {
	        if tmp % 2 == 0 {
		    tmp /= 2;
		} else if tmp % 3 == 0 {
		    tmp /= 3;
		} else if tmp % 5 == 0 {
		    tmp /= 5;
		} else {
		    return false
		}
	    }
	    true
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Ugly: {}", Solution::is_ugly(num));
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
