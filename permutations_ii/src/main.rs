extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
	let mut result: Vec<Vec<i32>> = Vec::new();
	let len: usize = nums.len();

	let mut index: usize = 0;
	while index < len {
	    let mut tmp_result: Vec<Vec<i32>> = Vec::new();
	    if result.is_empty() {
	        tmp_result.push(nums.clone());
	    } else {
		while !result.is_empty() {
		    tmp_result.push(result.pop().unwrap());
		}
	    }

	    for v in tmp_result {
                let mut tmp: Vec<i32> = v.clone();
                qsi32::quick_sort(&mut tmp, index, len - 1);
		result.push(tmp.clone());

	        let mut tmp_i: usize = index + 1;
	        let mut tmp_v: i32 = tmp[index];
	        while tmp_i < len {
		    if tmp[tmp_i] != tmp_v {
		        let mut tmp_nums: Vec<i32> = tmp.clone();
		        let tmp_n: i32 = tmp_nums[index];
		        tmp_nums[index] = tmp_nums[tmp_i];
		        tmp_nums[tmp_i] = tmp_n;
		        result.push(tmp_nums);
		    }
		    tmp_v = tmp[tmp_i];
		    tmp_i += 1;
		}
	    }
	    index += 1;
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

    let result = Solution::permute_unique(nums);
    for v in result {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
