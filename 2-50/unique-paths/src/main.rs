use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 0 || n == 0 {
	    return 0
	}

        let mut matrix: Vec<Vec<i32>> = Vec::new();
	matrix.push(vec![1; m as usize]);
	let mut i: usize = 1;
        while i < n as usize {
	    let mut row: Vec<i32> = vec![1];
	    let mut j: usize = 1;
	    while j < m as usize {
	        row.push(row[j - 1] + matrix[i - 1][j]);
		j += 1;
	    }
	    matrix.push(row);
	    i += 1;
	}
	matrix[n as usize - 1][m as usize - 1]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut m: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
	    m = i32::from_str(&arg).expect("Error parse.");
	} else if 2 == index {
	    ret = 2;
	    let n = i32::from_str(&arg).expect("Error parse.");
	    println!("Paths: {}", Solution::unique_paths(m, n));
	    break;
	} else {
	    continue;
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
    }
}
