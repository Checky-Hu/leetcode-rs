use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut s: i32 = 0;
	let mut e: i32 = x;
	let mut result: i32 = 0;
	while s <= e {
	    result = s + (e - s) / 2;
	    let value: i64 = (result as i64) * (result as i64);
	    if value > x.into() {
	        if result >= 1 {
	            e = result - 1;
		} else {
		    break;
		}
	    } else if value < x.into() {
	        s = result + 1;
	    } else {
	        break;
	    }
	}
	let value: i64 = (result as i64) * (result as i64);
	if value > x.into() {
	    result - 1
	} else {
	    result
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let x: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Integer: {}", Solution::my_sqrt(x));
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
