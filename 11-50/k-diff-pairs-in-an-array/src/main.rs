extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let len: usize = nums.len();
	if len == 0 || len == 1 {
	    return 0
	}

	let mut tmp: Vec<i32> = nums.clone();
	qsi32::quick_sort(&mut tmp, 0, len - 1);
	let mut result: i32 = 0;
	let mut i: usize = 0;
	let mut pre_n: i32 = 0;
	while i < len - 1 {
	    if i == 0 || pre_n != tmp[i] {
	        let mut j: usize = i + 1;
		let target: i64 = (tmp[i] + k).into();
		while j < len {
		    if tmp[j] as i64 > target {
		        j = len;
			break;
		    } else if tmp[j] as i64 == target {
		        break;
		    } else {
		        j += 1;
		    }
		}
		if j < len {
		    result += 1;
		}
	        pre_n = tmp[i];
	    }
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            k = i32::from_str(&arg).expect("Error parse.");
	} else if 1 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    nums.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
	return;
    }

    println!("Pairs: {}", Solution::find_pairs(nums, k));
}
