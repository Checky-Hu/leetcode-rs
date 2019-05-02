use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
	let len: usize = nums.len();
	let mut i: usize = 0;
	let mut j: usize = 0;
	while j < len {
	    if nums[j] != 0 {
	        nums[i] = nums[j];
		i += 1;
	    }
	    j += 1;
	}
	while i < len {
	    nums[i] = 0;
	    i += 1;
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

    Solution::move_zeroes(&mut nums);
    for n in nums {
        print!("{} ", n);
    }
    print!("\n");
}
