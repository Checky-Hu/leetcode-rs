extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
	let mut result: Vec<Vec<i32>> = vec![Vec::new()];
	let len: usize = nums.len();

	let mut sorted_nums: Vec<i32> = nums.clone();
	qsi32::quick_sort(&mut sorted_nums, 0, len - 1);

	let mut index: usize = 0;
	let mut preceding_pos: usize = 0;
	let mut preceding_n: i32 = 0;
	while index < len {
	    let cur_n: i32 = sorted_nums[index];
	    let cur_len: usize = result.len();
	    if index == 0 || preceding_n != cur_n {
	        preceding_n = cur_n;
		preceding_pos = cur_len;
	    }

	    let mut i: usize = cur_len - preceding_pos;
	    while i < cur_len {
                let mut tmp: Vec<i32> = result[i].clone();
		tmp.push(cur_n);
		result.push(tmp);
		i += 1;
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

    let result = Solution::subsets(nums);
    for v in result {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
