use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row: usize = m.len();
        let column: usize = m[0].len();
        let mut result: Vec<Vec<i32>> = vec![vec![0; column]; row];
	let mut i: usize = 0;
	while i < row {
	    let mut j: usize = 0;
	    while j < column {
	        let mut count: i32 = 1;
	        result[i][j] = m[i][j];
		if i > 0 {
		    result[i][j] += m[i - 1][j];
		    count += 1;
		    if j > 0 {
		        result[i][j] += m[i - 1][j - 1];
		        count += 1;
		    }
		    if j + 1 < column {
		        result[i][j] += m[i - 1][j + 1];
		        count += 1;
		    }
		}
		if j > 0 {
		    result[i][j] += m[i][j - 1];
		    count += 1;
		}
		if j + 1 < column {
		    result[i][j] += m[i][j + 1];
		    count += 1;
		}
		if i + 1 < row {
		    result[i][j] += m[i + 1][j];
		    count += 1;
		    if j > 0 {
		        result[i][j] += m[i + 1][j - 1];
		        count += 1;
		    }
		    if j + 1 < column {
		        result[i][j] += m[i + 1][j + 1];
		        count += 1;
		    }
		}
		result[i][j] /= count;
	        j += 1;
	    }
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut m: Vec<Vec<i32>> = Vec::new();
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
		    m.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameters.");
	return;
    }

    let result: Vec<Vec<i32>> = Solution::image_smoother(m);
    for v in result {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
