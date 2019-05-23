use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut flags: Vec<bool> = vec![true; nums.len()];
	for i in nums {
	    flags[i as usize - 1] = false;
	}
	let mut result: Vec<i32> = Vec::new();
	let mut n: i32 = 1;
	for v in flags {
	    if v {
	        result.push(n);
	    }
	    n += 1;
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
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    nums.push(n);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let result: Vec<i32> = Solution::find_disappeared_numbers(nums);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
