use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        for v in grid {
	    for c in v {
	        print!("{} ", c);
	    }
	    print!("\n");
	}
	0
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => rows = i32::from_str(&arg).expect("Error parse."),
	    2 => columns = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        if number == 1 {
		    tmp_row.push('1');
		} else {
		    tmp_row.push('0');
		}
		if ret % columns == 0 {
		    grid.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameter.");
	return;
    }

    println!("Count: {}", Solution::num_islands(grid));
}
