use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
	let mut max: i32 = -1;
	let mut second_max: i32 = -1;
	let mut index: usize = 0;
	let mut i: usize = 0;
	for n in nums {
	    if n > max {
		second_max = max;
	        max = n;
		index = i;
	    } else if n > second_max {
	        second_max = n;
	    }
	    i += 1;
	}
	if max >= 2 * second_max {
	    index as i32
	} else {
	    -1
	}
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

    println!("Dominant index: {}", Solution::dominant_index(nums));
}

