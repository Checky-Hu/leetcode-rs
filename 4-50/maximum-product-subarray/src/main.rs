use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
	if nums.is_empty() {
	    return 0
	}

        let mut result: i32 = nums[0];
	let mut pre_min: i32 = nums[0];
	let mut pre_max: i32 = nums[0];
	let mut cur_min: i32;
	let mut cur_max: i32;
	let mut i: usize = 1;
	while i < nums.len() {
	    let tmp1: i32 = pre_min * nums[i];
	    let tmp2: i32 = pre_max * nums[i];
	    if tmp1 > tmp2 {
	        if tmp1 > nums[i] {
		    cur_min = if tmp2 > nums[i] {
		        nums[i]
		    } else {
		        tmp2
		    };
		    cur_max = tmp1;
		} else {
		    cur_min = tmp2;
		    cur_max = nums[i];
		}
	    } else {
	        if tmp1 > nums[i] {
		    cur_min = nums[i];
		    cur_max = tmp2;
		} else {
		    cur_min = tmp1;
		    cur_max = if tmp2 > nums[i] {
		        tmp2
		    } else {
		        nums[i]
		    };
		}
	    }

	    if cur_max > result {
	        result = cur_max;
	    }
	    pre_min = cur_min;
	    pre_max = cur_max;
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

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Product: {}", Solution::max_product(nums));
}
