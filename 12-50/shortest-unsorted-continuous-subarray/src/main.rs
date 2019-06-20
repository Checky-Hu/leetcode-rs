use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
	if len <= 1 {
	    return 0
	}

        let mut s: usize = 1;
	let mut e: usize = 0;
	let mut max: i32 = nums[0];
	let mut min: i32 = nums[len - 1];
	let mut i: usize = 1;
	while i < len {
	    if max < nums[i] {
	        max = nums[i];
	    }
	    if max > nums[i] {
	        e = i;
	    }
	    if min > nums[len - 1 - i] {
	        min = nums[len - 1 - i];
	    }
	    if min < nums[len - 1 - i] {
	        s = len - 1 - i;
	    }
	    i += 1;
	}
	(1 + e - s) as i32
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

    println!("Length of unsorted subarray: {}", Solution::find_unsorted_subarray(nums));
}
