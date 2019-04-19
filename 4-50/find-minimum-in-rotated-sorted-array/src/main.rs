use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
	if len == 0 {
	    return -1
	}

	let mut s: usize = 0;
	let mut e: usize = len - 1;
	while s <= e {
	    let mid: usize = s + (e - s) / 2;
	    if mid == s {
	        if nums[mid] > nums[e] {
	            return nums[e]
		} else {
		    return nums[mid]
		}
	    } else {
	        if nums[mid] > nums[s] {
		    if nums[mid] > nums[e] {
		        // s -> mid -> min -> e.
		        s = mid + 1;
		    } else {
		        // s(min) -> mid -> e.
		        return nums[s]
		    }
		} else {
		    // s -> min -> mid -> e.
		    e = mid;
		}
	    }
	}
	-1
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

    println!("Minimum: {}", Solution::find_min(nums));
}
