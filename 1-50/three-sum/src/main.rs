extern crate binarysearch;
extern crate quicksort;

use std::env;
use std::str::FromStr;
use binarysearch::bsi32;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
	if nums.len() < 3 {
	    return result
	}

	let mut tmp_nums: Vec<i32> = nums.clone();
	let len: usize = tmp_nums.len() - 1;
	qsi32::quick_sort(&mut tmp_nums, 0, len);

	let mut i: usize = 0;
	let mut preceding_i: i32 = 0;
	while i < len - 1 {
	    if i == 0 || tmp_nums[i] != preceding_i {
	        let mut j: usize = i + 1;
	        let mut preceding_j: i32 = 0;
	        while j < len {
		    if j == i + 1 || tmp_nums[j] != preceding_j {
	                let target: i32 = 0 - (tmp_nums[i] + tmp_nums[j]);
		        let index: i32 = bsi32::binary_search(&tmp_nums, j + 1, len, target);
		        if index >= 0 {
		            let tmp_vec: Vec<i32> = vec![tmp_nums[i], tmp_nums[j], tmp_nums[index as usize]];
		            result.push(tmp_vec);
		        }
		    }
		    preceding_j = tmp_nums[j];
		    j += 1;
	        }
	    }
	    preceding_i = tmp_nums[i];
	    i += 1;
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

    if 3 > ret {
        println!("Require at least three parameter.");
	return;
    }

    let result: Vec<Vec<i32>> = Solution::three_sum(nums);
    for v in result {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
