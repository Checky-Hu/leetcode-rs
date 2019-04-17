use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
	let len: usize = nums.len();
        if len == 0 || k == 0 {
	    return
	}

	let count: usize = k as usize % len;
	let mut tmp: Vec<i32> = nums.drain(0..(len - count)).collect();
	nums.append(&mut tmp);
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut k: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => k = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret = index;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    Solution::rotate(&mut nums, k);
    for n in nums {
        print!("{} ", n);
    }
    print!("\n");
}
