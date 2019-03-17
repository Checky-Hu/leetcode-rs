use std::env;
use std::str::Chars;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
	    return s
	}

        let mut vecs: Vec<String> = Vec::new();
	for _i in 0..num_rows {
	    vecs.push(String::new());
	}

	let tmp_bytes: Chars = s.chars();
	let mut index: usize = 0;
	let group_size: usize = 2 * (num_rows as usize - 1);
	for b in tmp_bytes {
	    let in_group_pos: usize = index % group_size;
	    if in_group_pos < num_rows as usize {
	        vecs[in_group_pos].push(b);
	    } else {
	        for i in 0..num_rows {
		    if i as usize == group_size - in_group_pos {
		        vecs[i as usize].push(b);
		    } else {
			vecs[i as usize].push(0 as char);
		    }
		}
	    }
	    index += 1;
	}

	let mut result: String = String::new();
	for str in vecs {
	    for b in str.chars() {
	        if b != 0 as char {
		    result.push(b);
		}
	    }
	}
	result
    }
}

fn main() {
    let mut result: usize = 0;
    let mut str: String = String::new();
    let num_rows: i32;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            str = arg;
	} else if index == 2 {
	    result = index;
            num_rows = i32::from_str(&arg).expect("Error parser");
            println!("String: {}", Solution::convert(str, num_rows));
	    break;
	} else {
	    continue;
	}
    }

    match result {
        0 => println!("Require at least two parameter."),
	_ => (),
    }
}
