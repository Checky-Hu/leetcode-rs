use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row: usize = a.len();
	let column: usize = a[0].len();
	let mut result: Vec<Vec<i32>> = vec![vec![0; row]; column];
	let mut i: usize = 0;
	while i < column {
	    let mut j: usize = 0;
	    while j < row {
	        result[i][j] = a[j][i];
		j += 1;
	    }
	    i += 1;
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
        println!("Require at least (arg1 * arg2) parameters.");
	return;
    }

    let result: Vec<Vec<i32>> = Solution::transpose(matrix);
    for v in result {
        for n in v {
	    print!("{} ", n);
	}
	print!("\n");
    }
}
