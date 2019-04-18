use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
	if nums.is_empty() {
	    return 0
	}

	let len: usize = nums.len();
	if len == 1 {
	    return nums[0]
	}

	let mut money: Vec<i32> = vec![0; len];
	let mut i: usize = 0;
	while i < nums.len() {
	    if i < 2 {
	        money[i] = nums[i];
	    } else if i == 2 {
	        money[i] = money[i - 2] + nums[i];
	    } else {
	        money[i] = if money[i - 2] > money[i - 3] {
		    money[i - 2] + nums[i]
		} else {
		    money[i - 3] + nums[i]
		};
	    }
	    i += 1;
	}
	if money[len - 1] > money[len - 2] {
	    money[len - 1]
	} else {
	    money[len - 2]
	}
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

    println!("Money: {}", Solution::rob(nums));
}
