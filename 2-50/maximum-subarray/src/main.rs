use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
	let mut preceding_sum: i32 = 0;
	let mut index: usize = 0;
	for n in nums {
	    if index == 0 {
	        preceding_sum = n;
	        result = n;
	    } else {
		preceding_sum = if preceding_sum > 0 {
		    n + preceding_sum
		} else {
		    n
		};
		if preceding_sum > result {
		    result = preceding_sum;
		}
	    }
	    index += 1;
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

    println!("Sum: {}", Solution::max_sub_array(nums));
}
