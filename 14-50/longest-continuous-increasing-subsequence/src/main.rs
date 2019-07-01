use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
	let mut result: i32 = 0;
	let mut count: i32 = 0;
	let mut i: usize = 0;
	while i < nums.len() {
	    if i < 1 || nums[i] > nums[i - 1] {
	        count += 1;
	    } else {
	        if count > result {
		    result = count;
		}
	        count = 1;
	    }
	    i += 1;
	}
	if count > result {
            result = count;
	}
	result
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

    println!("Length: {}", Solution::find_length_of_lcis(nums));
}

