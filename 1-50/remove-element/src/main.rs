use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut b_index: usize = 0;
        let mut f_index: usize = 0;
	while f_index < nums.len() {
	    if nums[f_index] != val {
	        nums[b_index] = nums[f_index];
		b_index += 1;
	    }
	    f_index += 1;
	}
	let result: i32 = b_index as i32;
	while b_index < f_index {
	    nums.pop();
	    b_index += 1;
	}
	result
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

    let len: i32 = Solution::remove_element(&mut vector, target);
    for n in vector {
        print!("{} ", n);
    }
    print!("\nSize: {}\n", len);
}
