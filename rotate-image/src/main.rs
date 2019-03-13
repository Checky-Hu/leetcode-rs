use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut vectors: Vec<Vec<i32>> = Vec::new();
    let mut tmp_vec: Vec<i32> = Vec::new();
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => n = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        tmp_vec.push(number);
                ret += 1;
	        if ret == n {
		    ret = 0;
	            vectors.push(tmp_vec);
		    tmp_vec = Vec::new();
		}
	    },
	}
    }

    if 0 == n {
        println!("Require at least two parameter.");
	return;
    }

    Solution::rotate(&mut vectors);
    for v in vectors {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
