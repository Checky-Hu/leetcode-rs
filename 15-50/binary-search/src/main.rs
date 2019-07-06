use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
	let mut i: usize = 0;
	let mut j: usize = nums.len() - 1;
	while i <= j {
	    let index: usize = i + (j - i) / 2;
	    if nums[index] == target {
	        return index as i32
	    } else if nums[index] < target {
	        i = index + 1;
	    } else {
	        if index >= 1 {
	            j = index - 1;
		} else {
		    break;
		}
	    }
	}
	-1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut target: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
	return;
    }

    println!("Index: {}", Solution::search(nums, target));
}

