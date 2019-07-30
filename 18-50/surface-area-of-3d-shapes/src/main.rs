use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let row: usize = grid.len();
	let column: usize = grid[0].len();
	let mut result: i32 = 0;
	let mut i: usize = 0;
	while i < row {
	    let mut j: usize = 0;
	    while j < column {
	        let mut k: i32 = 1;
		while k <= grid[i][j] {
		    // bottom & top & left & right & front & back
		    if k == 1 {
		        result += 1;
		    }
		    if k == grid[i][j] {
		        result += 1;
		    }
		    if j == 0 || grid[i][j - 1] < k {
		        result += 1;
		    }
		    if j == column - 1 || grid[i][j + 1] < k {
		        result += 1;
		    }
		    if i == 0 || grid[i - 1][j] < k {
		        result += 1;
		    }
		    if i == row - 1 || grid[i + 1][j] < k {
		        result += 1;
		    }
		    k += 1;
		}
		j += 1;
	    }
	    i += 1;
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

    println!("Area: {}", Solution::surface_area(grid));
}
