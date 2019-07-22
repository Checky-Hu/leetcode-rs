use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
	for v in a {
	    let len: usize = v.len();
	    let mut tmp: Vec<i32> = Vec::new();
	    let mut i: usize = len - 1;
	    loop {
	        tmp.push(if v[i] == 1 {
		    0
		} else {
		    1
		});
		if i == 0 {
		    break;
		} else {
		    i -= 1;
		}
	    }
	    result.push(tmp);
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<Vec<i32>> = Vec::new();
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
		    a.push(tmp_row);
		    tmp_row = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameters.");
	return;
    }

    let result: Vec<Vec<i32>> = Solution::flip_and_invert_image(a);
    for v in result {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
