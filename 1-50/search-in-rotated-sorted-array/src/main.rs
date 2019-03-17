use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len: usize = nums.len();
	if len == 0 {
	    return -1
	}

        let mut index: usize;
	let mut s: usize = 0;
	let mut e: usize = len - 1;
	while s <= e {
	    index = s + (e - s) / 2;
	    if nums[index] == target {
	        return index as i32
	    } else if nums[index] < nums[e] {
	        if nums[index] < target && target <= nums[e] {
		    s = index + 1;
		} else {
		    if index >= 1 {
		        e = index - 1;
		    } else {
		        break;
		    }
		}
	    } else {
	        if nums[index] > target && target >= nums[s] {
		    if index >= 1 {
		        e = index - 1;
		    } else {
		        break;
		    }
		} else {
		    s = index + 1;
		}
	    }
	}
	-1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut vector: Vec<i32> = Vec::new();
    let mut target: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => target = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret = index;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        vector.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    println!("Position: {}", Solution::search(vector, target));
}
