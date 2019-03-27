use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
	if matrix.is_empty() {
	    return result
	}

	let mut r_s: usize = 0;
	let mut r_e: usize = matrix.len() - 1;
	let mut c_s: usize = 0;
	let mut c_e: usize = matrix[0].len() - 1;
	loop {
	    if r_s == r_e && c_s == c_e {
	        result.push(matrix[r_s][c_s]);
	    } else if r_s == r_e {
	        for x in c_s..=c_e {
		    result.push(matrix[r_s][x]);
		}
	    } else if c_s == c_e {
	        for y in r_s..=r_e {
		    result.push(matrix[y][c_s]);
		}
	    } else {
	        let mut x: usize = c_s;
	        while x < c_e {
	            result.push(matrix[r_s][x]);
		    x += 1;
	        }
	        let mut y: usize = r_s;
	        while y < r_e {
	            result.push(matrix[y][c_e]);
		    y += 1;
	        }
	        x = c_e;
	        while x > c_s {
	            result.push(matrix[r_e][x]);
		    if x > c_s + 1 {
		        x -= 1;
		    } else {
		        break;
		    }
	        }
	        y = r_e;
	        while y > r_s {
	            result.push(matrix[y][c_s]);
		    if y > r_s + 1 {
		        y -= 1;
		    } else {
		        break;
		    }
	        }
	    }
	    r_s += 1;
	    if r_e >= r_s + 1 {
	        r_e -= 1;
	    } else {
	        break;
	    }
	    c_s += 1;
	    if c_e >= c_s + 1 {
	        c_e -= 1;
	    } else {
	        break;
	    }
	}
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => rows = i32::from_str(&arg).expect("Error parse."),
	    2 => columns = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        tmp_row.push(number);
		if ret % columns == 0 {
		    matrix.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameter.");
	return;
    }

    let result: Vec<i32> = Solution::spiral_order(matrix);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
