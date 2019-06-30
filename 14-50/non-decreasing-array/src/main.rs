use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
	let len: usize = nums.len();
        let mut found: bool = false;
	let mut i: usize = 1;
	while i < len {
	    if nums[i - 1] > nums[i] {
	        if found {
		    return false
		} else {
	            if i == len - 1 || nums[i + 1] >= nums[i - 1] {
		        // modify nums[i]
		        found = true;
		    } else if i < 2 || nums[i] >= nums[i - 2] {
		        // modify nums[i - 1]
		        found = true;
		    } else {
		        return false
		    }
		}
	    }
	    i += 1;
	}
	true
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

    println!("Non-decreasing: {}", Solution::check_possibility(nums));
}

