use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn set_zeros(matrix: &mut Vec<Vec<i32>>) {
	if matrix.is_empty() {
	    return
	}

	let mut is_first_row_zero: bool = false;
	for i in 0..matrix[0].len() {
	    if matrix[0][i] == 0 {
	        is_first_row_zero = true;
		break;
	    }
	}
	let mut is_first_column_zero: bool = false;
	for i in 0..matrix.len() {
	    if matrix[i][0] == 0 {
	        is_first_column_zero = true;
		break;
	    }
	}

	let mut i: usize = 1;
	let mut j: usize;
	while i < matrix.len() {
	    j = 1;
	    while j < matrix[i].len() {
	        if matrix[i][j] == 0 {
		    matrix[i][0] = 0;
		    matrix[0][j] = 0;
		}
		j += 1;
	    }
	    i += 1;
	}
	i = 1;
	while i < matrix.len() {
	    j = 1;
	    while j < matrix[i].len() {
	        if matrix[i][0] == 0 || matrix[0][j] == 0 {
		    matrix[i][j] = 0;
		}
		j += 1;
	    }
	    i += 1;
	}

	if is_first_row_zero {
	    for i in 0..matrix[0].len() {
	        matrix[0][i] = 0;
	    }
	}
	if is_first_column_zero {
	    for i in 0..matrix.len() {
	        matrix[i][0] = 0;
	    }
	}
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

    Solution::set_zeros(&mut matrix);
    for v in matrix {
        for i in v {
            print!("{} ", i);
	}
	print!("\n");
    }
}
