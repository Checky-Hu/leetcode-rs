extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let mut tmp: Vec<i32> = nums.clone();
	qsi32::quick_sort(&mut tmp, 0, len - 1);
	let product1: i32 = tmp[len - 3] * tmp[len - 2];
	if tmp[0] < 0 && tmp[1] < 0 {
	    let product2: i32 = tmp[0] * tmp[1];
	    if product2 > product1 {
	        tmp[len - 1] * product2
	    } else {
	        tmp[len - 1] * product1
	    }
	} else {
	    tmp[len - 1] * product1
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Maximum product: {}", Solution::maximum_product(nums));
}

