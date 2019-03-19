use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.is_empty() {
	    return
	}

	let len: usize = nums.len();
        let mut index: usize = len - 1;
	while index > 0 {
	    if nums[index - 1] < nums[index] {
	        let mut tmp_i: usize = index + 1;
		while tmp_i < len {
		    if nums[tmp_i] <= nums[index - 1] {
		        break;
		    }
		    tmp_i += 1;
		}

	        let tmp: i32 = nums[index - 1];
		nums[index - 1] = nums[tmp_i - 1];
		nums[tmp_i - 1] = tmp;
	        break;
	    }
	    index -= 1;
	}

	let mut offset: usize = 0;
	while offset < (len - index) / 2 {
	    let tmp: i32 = nums[index + offset];
	    nums[index + offset] = nums[len - 1 - offset];
	    nums[len - 1 - offset] = tmp;
	    offset += 1;
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

    Solution::next_permutation(&mut nums);
    for n in nums {
        print!("{} ", n);
    }
    print!("\n");
}
