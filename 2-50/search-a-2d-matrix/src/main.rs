extern crate binarysearch;

use std::env;
use std::str::FromStr;
use binarysearch::bsi32;

struct Solution {
}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
	    return false
	}

	let mut tmp_column: Vec<i32> = Vec::new();
	for i in 0..matrix.len() {
	    tmp_column.push(matrix[i][0]);
	}

	let mut row: i32 = bsi32::binary_search_for_pos(&tmp_column, 0, matrix.len() - 1, target);
	if row == -1 {
	    false
	} else {
	    if row == -2 {
	        row = matrix.len() as i32 - 1;
	    }

	    let column: i32 = bsi32::binary_search_for_pos(&matrix[row as usize], 0, matrix[row as usize].len() - 1, target);
	    if column < 0 {
	        false
	    } else {
	        matrix[row as usize][column as usize] == target
	    }
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut target: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => target = i32::from_str(&arg).expect("Error parse."),
	    2 => rows = i32::from_str(&arg).expect("Error parse."),
	    3 => columns = i32::from_str(&arg).expect("Error parse."),
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

    println!("Found: {}", Solution::search_matrix(matrix, target));
}
