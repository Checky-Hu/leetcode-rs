use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len: usize = nums.len();
        let mut front: Vec<i32> = vec![1; len];
	let mut i: usize = 0;
	while i < len - 1 {
	    front[i + 1] = front[i] * nums[i];
	    i += 1;
	}
        let mut end: Vec<i32> = vec![1; len];
	i = len - 1;
	while i > 0 {
	    end[i - 1] = end[i] * nums[i];
	    i -= 1;
	}
	let mut result: Vec<i32> = Vec::new();
	i = 0;
	while i < len {
	    result.push(front[i] * end[i]);
	    i += 1;
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
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    nums.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let result: Vec<i32> = Solution::product_except_self(nums);
    for i in result {
        print!("{} ", i);
    }
    print!("\n");
}
