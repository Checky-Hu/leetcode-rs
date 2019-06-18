use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let row: usize = nums.len();
	let column: usize = nums[0].len();
	if row * column != (r * c) as usize {
	    return nums
	}

	let mut result: Vec<Vec<i32>> = Vec::new();
	let mut tmp_row: Vec<i32> = Vec::new();
	for v in nums {
	    for n in v {
	        tmp_row.push(n);
		if tmp_row.len() == c as usize {
		    result.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<Vec<i32>> = Vec::new();
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => r = i32::from_str(&arg).expect("Error parse."),
	    2 => c = i32::from_str(&arg).expect("Error parse."),
	    3 => rows = i32::from_str(&arg).expect("Error parse."),
	    4 => columns = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        tmp_row.push(number);
		if ret % columns == 0 {
		    nums.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg3 * arg4) parameters.");
	return;
    }

    let result: Vec<Vec<i32>> = Solution::matrix_reshape(nums, r, c);
    for v in result {
        for i in v {
            print!("{} ", i);
	}
	print!("\n");
    }
}
