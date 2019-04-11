extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
	let mut result: Vec<Vec<i32>> = Vec::new();
	let len: usize = nums.len();

	let mut sorted_nums: Vec<i32> = nums.clone();
	qsi32::quick_sort(&mut sorted_nums, 0, len - 1);

	let mut index: usize = 0;
	while index < len {
	    let mut tmp_result: Vec<Vec<i32>> = Vec::new();
	    while !result.is_empty() {
		tmp_result.push(result.pop().unwrap());
	    }

	    result.push(vec![sorted_nums[index]]);
	    for v in tmp_result {
                let mut tmp: Vec<i32> = v.clone();
		result.push(tmp.clone());
		tmp.push(sorted_nums[index]);
		result.push(tmp);
	    }
	    index += 1;
	}
	result.push(Vec::new());
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
