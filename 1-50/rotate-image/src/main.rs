use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
	if len == 0 {
	    return;
	}

	let mut index: usize = 0;
	let mut cur_len: usize = len - 1;
	loop {
	    let mut offset: usize = 0;
	    while offset < cur_len {
	        let tmp: i32 = matrix[index][index + offset];
		matrix[index][index + offset] = matrix[index + cur_len - offset][index];
		matrix[index + cur_len - offset][index] = matrix[index + cur_len][index + cur_len - offset];
		matrix[index + cur_len][index + cur_len - offset] = matrix[index + offset][index + cur_len];
		matrix[index + offset][index + cur_len] = tmp;
		offset += 1;
	    }
	    index += 1;
	    if cur_len <= 2 {
	        break;
	    } else {
	        cur_len -= 2;
	    }
	}
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
