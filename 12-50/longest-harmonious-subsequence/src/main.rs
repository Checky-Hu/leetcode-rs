extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
	if len <= 1 {
	    return 0
	}

	let mut tmp: Vec<i32> = nums.clone();
	qsi32::quick_sort(&mut tmp, 0, len - 1);
	let mut result: i32 = 0;
	let mut pre_num: i32 = tmp[0];
	let mut pre_count: i32 = 1;
	let mut cur_count: i32 = 0;
	let mut i: usize = 1;
	while i < len {
	    if tmp[i] == pre_num {
	        pre_count += 1;
	    } else if tmp[i] == pre_num + 1 {
	        cur_count += 1;
	    } else {
	        if cur_count > 0 && pre_count + cur_count > result {
		    result = pre_count + cur_count;
		}
		if tmp[i] == tmp[i - 1] + 1 {
		    pre_num = tmp[i - 1];
		    pre_count = cur_count;
		    cur_count = 1;
		} else {
		    pre_num = tmp[i];
		    pre_count = 1;
		    cur_count = 0;
		}
	    }
	    i += 1;
	}
	if cur_count > 0 && pre_count + cur_count > result {
	    result = pre_count + cur_count;
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

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Length of lhs: {}", Solution::find_lhs(nums));
}
