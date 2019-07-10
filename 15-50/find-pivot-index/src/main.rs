use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
	let mut left: i32 = 0;
	let mut right: i32 = 0;
	let mut i: usize = 0;
	while i < nums.len() {
	    if i == 0 {
	        let mut j: usize = 1;
		while j < len {
		    right += nums[j];
		    j += 1;
		}
	    } else {
	        left += nums[i - 1];
		right -= nums[i];
	    }
	    if left == right {
	        return i as i32
	    } else {
	        i += 1;
	    }
	}
	-1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Pivot index: {}", Solution::pivot_index(nums));
}

