use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let len: i32 = nums.len() as i32;
        let mut min: i32 = nums[0];
	let mut sum: i64 = 0;
	for n in nums {
	    sum += n as i64;
	    if n < min {
	        min = n;
	    }
	}
	(sum - (min * len) as i64) as i32
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

    println!("Min moves: {}", Solution::min_moves(nums));
}
