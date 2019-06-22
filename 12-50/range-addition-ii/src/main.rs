use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() {
	    return m * n
	}

	let mut min_a: i32 = ops[0][0];
	let mut min_b: i32 = ops[0][1];
        for v in ops {
	    if v[0] < min_a {
	        min_a = v[0];
	    }
	    if v[1] < min_b {
	        min_b = v[1];
	    }
	}
        min_a * min_b
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut ops: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => m = i32::from_str(&arg).expect("Error parse."),
	    2 => n = i32::from_str(&arg).expect("Error parse."),
	    3 => rows = i32::from_str(&arg).expect("Error parse."),
	    4 => columns = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        tmp_row.push(number);
		if ret % columns == 0 {
		    ops.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg3 * arg4) parameters.");
	return;
    }

    println!("Max count: {}", Solution::max_count(m, n, ops));
}
