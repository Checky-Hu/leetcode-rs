extern crate binarysearch;

use std::env;
use std::str::FromStr;
use binarysearch::bsi32;

struct Solution {
}

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut tmp: Vec<i32> = Vec::new();
	let mut i: usize = 0;
	loop {
	    let product: usize = i * i;
	    if product == c as usize {
	        return true
	    } else if product < c as usize {
	        tmp.push(product as i32);
	        i += 1;
	    } else {
	        break;
	    }
	}

	let mut j: usize = 0;
	while j < i {
	    if tmp[j] < c {
		if 0 <= bsi32::binary_search(&tmp, 0, i - 1, c - tmp[j]) {
		    return true
		} else {
		    j += 1;
		}
	    } else {
		break;
	    }
	}
        false
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
	    ret = 1;
	    let c: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Sum of squares: {}", Solution::judge_square_sum(c));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
