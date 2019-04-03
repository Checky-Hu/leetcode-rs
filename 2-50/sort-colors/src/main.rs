extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        qsi32::quick_sort(nums, 0, nums.len() - 1);
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

    Solution::sort_colors(&mut nums);
    for n in nums {
        print!("{} ", n);
    }
    print!("\n");
}
