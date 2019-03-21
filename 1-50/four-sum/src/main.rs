extern crate binarysearch;
extern crate quicksort;

use std::env;
use std::str::FromStr;
use binarysearch::bsi32;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
	if nums.len() < 4 {
	    return result
	}

	let mut tmp_nums: Vec<i32> = nums.clone();
	let len: usize = tmp_nums.len() - 1;
	qsi32::quick_sort(&mut tmp_nums, 0, len);

	let mut i: usize = 0;
	let mut preceding_i: i32 = 0;
	while i < len - 2 {
	    if i == 0 || tmp_nums[i] != preceding_i {
	        let mut j: usize = i + 1;
	        let mut preceding_j: i32 = 0;
	        while j < len - 1 {
		    if j == i + 1 || tmp_nums[j] != preceding_j {
		        let mut k: usize = j + 1;
			let mut preceding_k: i32 = 0;
			while k < len {
			    if k == j + 1 || tmp_nums[k] != preceding_k {
	                        let tmp: i32 = target - tmp_nums[i] - tmp_nums[j] - tmp_nums[k];
		                let index: i32 = bsi32::binary_search(&tmp_nums, k + 1, len, tmp);
		                if index >= 0 {
		                    let tmp_vec: Vec<i32> = vec![tmp_nums[i], tmp_nums[j], tmp_nums[k], tmp_nums[index as usize]];
		                    result.push(tmp_vec);
		                }
			    }
			    preceding_k = tmp_nums[k];
			    k += 1;
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
    let mut target: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => target = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret = index;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 4 > ret {
        println!("Require at least four parameter.");
	return;
    }

    let result: Vec<Vec<i32>> = Solution::four_sum(nums, target);
    for v in result {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
