extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut result: i32 = 0;
	let mut has_val: bool = false;
	if nums.len() < 3 {
	    for n in nums {
	        result += n;
	    }
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
	                let tmp_target: i32 = target - (tmp_nums[i] + tmp_nums[j]);
			let mut k: usize = j + 1;
			while k <= len {
			    if tmp_nums[k] == tmp_target {
			        return target
			    } else if tmp_nums[k] > tmp_target {
			        break;
			    } else {
			        k += 1;
			    }
			}
			let tmp_res: i32 = if k > len {
			    tmp_nums[i] + tmp_nums[j] + tmp_nums[len]
			} else {
			    if k == j + 1 {
			        tmp_nums[i] + tmp_nums[j] + tmp_nums[k]
			    } else {
			        if tmp_nums[k] - tmp_target >= tmp_target - tmp_nums[k - 1] {
			            tmp_nums[i] + tmp_nums[j] + tmp_nums[k - 1]
				} else {
			            tmp_nums[i] + tmp_nums[j] + tmp_nums[k]
				}
			    }
			};

			if has_val {
			    if i32::abs(target - result) > i32::abs(target - tmp_res) {
			        result = tmp_res;
			    }
			} else {
			    result = tmp_res;
			    has_val = true;
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
        println!("Require at least 4 parameter.");
	return;
    }

    println!("Integer: {}", Solution::three_sum_closest(nums, target));
}
