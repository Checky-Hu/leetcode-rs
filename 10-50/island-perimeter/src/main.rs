use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
	if grid.is_empty() || grid[0].is_empty() {
	    return 0
	}

	let row: usize = grid.len();
	let col: usize = grid[0].len();
        let mut result: i32 = 0;
	let mut i: usize = 0;
	while i < row {
	    let mut j: usize = 0;
	    while j < col {
	        if grid[i][j] == 1 {
		    result += 4;
		    if i > 0 && grid[i - 1][j] == 1 {
		        result -= 1;
		    }
		    if i + 1 < row && grid[i + 1][j] == 1 {
		        result -= 1;
		    }
		    if j > 0 && grid[i][j - 1] == 1 {
		        result -= 1;
		    }
		    if j + 1 < col && grid[i][j + 1] == 1 {
		        result -= 1;
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
    let mut n: i32 = 0;
    let mut tmp: Vec<i32> = Vec::new();
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            n = i32::from_str(&arg).expect("Error parse.");
	    if n <= 0 {
	        break;
	    }
	} else if 1 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    tmp.push(number);
	    if tmp.len() == n as usize {
	        grid.push(tmp);
		tmp = Vec::new();
	    }
	}
    }

    if 0 == ret || ret % n != 0 {
        println!("Require at least (m * n) parameters.");
	return;
    }

    println!("Perimeter: {}", Solution::island_perimeter(grid));
}
