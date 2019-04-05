use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; row_index as usize + 1];
	result[0] = 1;
	let mut i: usize = 1;
	while i < row_index as usize + 1 {
	    let mut j: usize = i;
	    while j > 0 {
	        result[j] += result[j - 1];
		j -= 1;
	    }
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let row_index: i32 = i32::from_str(&arg).expect("Error parse.");
	    let result: Vec<i32> = Solution::get_row(row_index);
	    for i in result {
                print!("{} ", i);
	    }
	    print!("\n");
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }
}
