use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
	let len: usize = nums.len();
	if len < 3 {
	    return false
	}

        let mut i: usize = 0;
	let mut min: i32 = i32::max_value();
	while i < len - 1 {
	    if nums[i] < min {
	        min = nums[i];
	    }
	    if min != nums[i] {
	        let mut j: usize = len - 1;
	        while j > i {
	            if min < nums[j] && nums[i] > nums[j] {
	                return true
		    }
		    j -= 1;
	        }
	    }
	    i += 1;
	}
	false
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

    println!("132 pattern: {}", Solution::find132pattern(nums));
}
