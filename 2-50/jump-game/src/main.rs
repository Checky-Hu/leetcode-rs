use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
	let mut i: usize = 0;
	let mut last: usize = 0;
	while i <= last {
	    let tmp_reached: usize = i + nums[i] as usize;
	    if tmp_reached >= nums.len() - 1 {
	        return true
	    } else {
	        if last < tmp_reached {
	            last = tmp_reached;
	        }
	        i += 1;
	    }
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

    println!("Reach: {}", Solution::can_jump(nums));
}
