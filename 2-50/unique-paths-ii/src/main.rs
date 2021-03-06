use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n: usize = obstacle_grid.len();
        if n == 0 {
	    return 0
	}
        let m: usize = obstacle_grid[0].len();
        if m == 0 {
	    return 0
	}

        let mut matrix: Vec<Vec<i32>> = Vec::new();
        for i in 0..n {
	    let mut row: Vec<i32> = Vec::new();
	    for j in 0..m {
	        let tmp: i32 = if obstacle_grid[i][j] == 1 {
		    0
		} else {
		    if i == 0 {
		        if j == 0 {
			    1
			} else {
			    row[j - 1]
			}
		    } else {
		        if j == 0 {
			    matrix[i - 1][j]
			} else {
			    matrix[i - 1][j] + row[j - 1]
			}
		    }
		};
	        row.push(tmp);
	    }
	    matrix.push(row);
	}
	matrix[n - 1][m - 1]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut obstacle_grid: Vec<Vec<i32>> = Vec::new();
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
		    obstacle_grid.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameter.");
	return;
    }

    println!("Paths: {}", Solution::unique_paths_with_obstacles(obstacle_grid));
}
