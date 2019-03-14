use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
	let len: usize = nums.len();
	let mut index: usize = 0;
	while index < len {
	    let mut tmp_result: Vec<Vec<i32>> = Vec::new();
	    if result.is_empty() {
	        tmp_result.push(nums.clone());
	    } else {
		while !result.is_empty() {
		    tmp_result.push(result.pop().unwrap());
		}
	    }

	    for v in tmp_result {
		result.push(v.clone());
	        let mut tmp_i: usize = index + 1;
	        while tmp_i < len {
		    let mut tmp_nums: Vec<i32> = v.clone();
		    let tmp_n: i32 = tmp_nums[index];
		    tmp_nums[index] = tmp_nums[tmp_i];
		    tmp_nums[tmp_i] = tmp_n;
		    result.push(tmp_nums);
		    tmp_i += 1;
		}
	    }
	    index += 1;
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

    let result = Solution::permute(nums);
    for v in result {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
