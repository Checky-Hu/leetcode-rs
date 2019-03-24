use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
	let mut result: i32 = 0;
	let mut i: usize = 0;
	let mut last: usize = 0;
	let mut cur_reached: usize = 0;
	while i < nums.len() {
	    if i > last {
	        last = cur_reached;
		result += 1;
	    }
	    let tmp_reached: usize = i + nums[i] as usize;
	    if cur_reached < tmp_reached {
	        cur_reached = tmp_reached;
	    }
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

    println!("Jumps: {}", Solution::jump(nums));
}
