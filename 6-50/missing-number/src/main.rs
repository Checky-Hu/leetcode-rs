use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut i: i32 = 0;
	let mut result: i32 = 0;
	while (i as usize) < nums.len() {
	    result ^= (i + 1) ^ nums[i as usize];
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

    println!("Missing: {}", Solution::missing_number(nums));
}
