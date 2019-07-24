use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
	let row: usize = grid.len();
	let column: usize = grid[0].len();
	let mut i: usize = 1;
	while i < row - 1 {
	    let mut j: usize = 1;
	    while j < column - 1 {
	        if grid[i][j] == 5 {
		    let mut flag: Vec<bool> = vec![false; 9];
		    let mut is_magic: bool = true;
		    let mut r: usize = i - 1;
		    // Check horizontal.
		    while r <= i + 1 {
		        let mut sum: i32 = 0;
		        let mut c: usize = j - 1;
			while c <= j + 1 {
			    if 0 < grid[r][c] && grid[r][c] < 10 && !flag[grid[r][c] as usize - 1] {
			        flag[grid[r][c] as usize - 1] = true;
				sum += grid[r][c];
			        c += 1;
			    } else {
			        is_magic = false;
				break;
			    }
			}
			if !is_magic || sum != 15 {
			    break;
			} else {
		            r += 1;
			}
		    }
		    if r > i + 1 {
		        // Check vertical.
		        r = j - 1;
			while r <= j + 1 {
			    let mut sum: i32 = 0;
			    let mut c: usize = i - 1;
			    while c <= i + 1 {
			        sum += grid[c][r];
				c += 1;
			    }
			    if sum != 15 {
			        break;
			    } else {
			        r += 1;
			    }
			}
			if r > j + 1 {
			    result += 1;
			}
		    }
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

    println!("Count: {}", Solution::num_magic_squares_inside(grid));
}
