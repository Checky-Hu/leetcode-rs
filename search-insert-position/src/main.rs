use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut index: usize = 0;
	while index < nums.len() {
	    if nums[index] >= target {
		break;
	    } else {
	        index += 1;
	    }
	}
	index as i32
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

    let position: i32 = Solution::search_insert(vector, target);
    println!("Position: {}", position);
}
