use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
	let mut count: i32 = 0;
	for n in nums {
	    if n == 1 {
	        count += 1;
	    } else {
	        if count > result {
		    result = count;
		}
	        count = 0;
	    }
	}
	if count > result {
	    result = count;
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

    println!("Count: {}", Solution::find_max_consecutive_ones(nums));
}
