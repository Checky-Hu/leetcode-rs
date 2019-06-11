extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
	let len: usize = nums.len();
	if len == 0 {
	    return result
	}

        let mut tmp: Vec<i32> = nums.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
	for n in nums {
	    if n == tmp[len - 1] {
	        result.push("Gold Medal".to_string());
	    } else if len >= 2 && n == tmp[len - 2] {
	        result.push("Silver Medal".to_string());
	    } else if len >= 3 && n == tmp[len - 3] {
	        result.push("Bronze Medal".to_string());
	    } else if len >= 4 {
	        let mut i: usize = 0;
		while i < len {
		    if tmp[i] == n {
		        break;
		    } else {
		        i += 1;
		    }
		}
		result.push((len - i).to_string());
	    }
	}
	result
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

    let result: Vec<String> = Solution::find_relative_ranks(nums);
    for s in result {
        println!("{}", s);
    }
}
