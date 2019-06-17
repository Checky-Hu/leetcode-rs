extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
	if len == 0 {
	    return 0
	}

	let mut tmp: Vec<i32> = nums.clone();
	qsi32::quick_sort(&mut tmp, 0, len - 1);
	let mut result: i32 = 0;
	let mut i: usize = 0;
	while i < len - 1 {
	    result += tmp[i];
	    i += 2;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    nums.push(number);
	}
    }

    if 2 > ret {
        println!("Require at least two parameters.");
	return;
    }

    println!("Sum: {}", Solution::array_pair_sum(nums));
}
