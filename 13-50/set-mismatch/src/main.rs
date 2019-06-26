use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
	let len: usize = nums.len();
        let mut tmp: Vec<i32> = vec![0; len];
	for n in nums {
	    tmp[n as usize - 1] += 1;
	}
	let mut result: Vec<i32> = vec![0; 2];
	for i in 0..len {
	    if tmp[i] == 2 {
	        result[0] = i as i32 + 1;
	    } else if tmp[i] == 0 {
	        result[1] = i as i32 + 1;
	    }
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

    let result: Vec<i32> = Solution::find_error_nums(nums);
    println!("Mismatch array: [{}, {}]", result[0], result[1]);
}

