use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let row: usize = matrix.len();
	let column: usize = matrix[0].len();
	let mut i: usize = 0;
	while i < column - 1 {
	    let target: i32 = matrix[0][i];
	    let mut tmp_i: usize = i + 1;
	    let mut j: usize = 1;
	    while j < row && tmp_i < column {
	        println!("j: {}, tmp_i: {}, v: {}", j, tmp_i, matrix[j][tmp_i]);
	        if matrix[j][tmp_i] != target {
		    return false
		} else {
		    tmp_i += 1;
		    j += 1;
		}
	    }
	    i += 1;
	}

	i = 1;
	while i < row - 1 {
	    let target: i32 = matrix[i][0];
	    let mut tmp_i: usize = i + 1;
	    let mut j: usize = 1;
	    while j < column && tmp_i < row {
	        println!("j: {}, tmp_i: {}, v: {}", j, tmp_i, matrix[tmp_i][j]);
	        if matrix[tmp_i][j] != target {
		    return false
		} else {
		    tmp_i += 1;
		    j += 1;
		}
	    }
	    i += 1;
	}
        true
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
        println!("Require at least (arg1 * arg2) parameters.");
	return;
    }

    println!("Is toeplitz: {}", Solution::is_toeplitz_matrix(matrix));
}
