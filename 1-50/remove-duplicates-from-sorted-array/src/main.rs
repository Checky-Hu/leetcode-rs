use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut b_index: usize = 0;
        let mut f_index: usize = 0;
	while f_index < nums.len() {
	    if f_index == 0 || nums[f_index] != nums[b_index - 1] {
	        nums[b_index] = nums[f_index];
		b_index += 1;
	    }
	    f_index += 1;
	}
	b_index as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut vector: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    vector.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let len: i32 = Solution::remove_duplicates(&mut vector);
    for n in vector {
        print!("{} ", n);
    }
    print!("\nSize: {}\n", len);
}
