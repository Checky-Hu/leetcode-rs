use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len: usize = nums.len();
	if len == 0 {
	    return vec![-1, -1]
	}

	let mut s: usize = 0;
	let mut e: usize = len - 1;
	while s <= e {
	    let index: usize = s + (e - s) / 2;
	    if nums[index] == target {
		let mut i: usize = s;
		while i <= index {
		    if nums[i] == target {
		        break;
		    }
		    i += 1;
		}
		let mut j: usize = index;
		while j <= e {
		    if nums[j] != target {
		        break;
		    }
		    j += 1;
		}
	        return vec![i as i32, j as i32 - 1]
	    } else if nums[index] < target {
	        s = index + 1;
	    } else {
		if index >= 1 {
		    e = index - 1;
		} else {
		    break;
		}
	    }
	}
	vec![-1, -1]
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

    let result: Vec<i32> = Solution::search_range(vector, target);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
