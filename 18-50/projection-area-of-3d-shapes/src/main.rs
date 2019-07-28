use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let row: usize = grid.len();
	let column: usize = grid[0].len();
	let mut result: i32 = 0;
	let mut max_row: Vec<i32> = vec![0; row];
	let mut max_column: Vec<i32> = vec![0; column];
	let mut i: usize = 0;
	while i < row {
	    let mut j: usize = 0;
	    let mut max: i32 = 0;
	    while j < column {
	        if grid[i][j] != 0 {
		    result += 1;
		}
		if grid[i][j] > max_column[j] {
		    max_column[j] = grid[i][j];
		}
		if grid[i][j] > max {
		    max = grid[i][j];
		}
		j += 1;
	    }
	    max_row[i] = max;
	    i += 1;
	}
	for n in max_row {
	    result += n;
	}
	for n in max_column {
	    result += n;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
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
		    grid.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameters.");
	return;
    }

    println!("Area: {}", Solution::projection_area(grid));
}
