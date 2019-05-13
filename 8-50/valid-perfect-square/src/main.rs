use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut s: i32 = 0;
	let mut e: i32 = 46340;
	while s <= e {
	    let mid: i32 = s + (e - s) / 2;
	    let product: i32 = mid * mid;
	    if product == num {
	        return true
	    } else if product < num {
	        s = mid + 1;
	    } else {
	        e = mid - 1;
	    }
	}
	false
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Square: {}", Solution::is_perfect_square(num));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
