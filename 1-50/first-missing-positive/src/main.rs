extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
	    return 1;
	}

	let mut tmp: Vec<i32> = nums.clone();
	qsi32::quick_sort(&mut tmp, 0, nums.len() - 1);
	let mut index: usize = 0;
	let mut preceding_i: i32 = 1;
	while index < nums.len() {
	    if tmp[index] <= 0 {
	        index += 1;
	    } else {
	        if preceding_i == tmp[index] {
		    preceding_i += 1;
		    index += 1;
		} else if preceding_i - 1 == tmp[index] {
		    index += 1;
		} else {
		    break;
		}
	    }
	}
        preceding_i
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    nums.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Integer: {}", Solution::first_missing_positive(nums));
}
