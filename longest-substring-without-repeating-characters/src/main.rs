use std::env;

struct Solution {
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
	let mut index: usize = 0;
	let mut max_len: usize = 0;
	while index + max_len < bytes.len() {
            let mut flags: [bool; 256] = [false; 256];
	    let mut tmp_i: usize = 0;
	    let mut tmp_c: usize;
	    while index + tmp_i < bytes.len() {
	        tmp_c = bytes[index + tmp_i] as usize;
		if flags[tmp_c] == true {
		    break;
		} else {
		    flags[tmp_c] = true;
		    tmp_i += 1;
		}
	    }

	    if tmp_i > max_len {
	        max_len = tmp_i;
	    }
	    index += 1;
	}

	max_len as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            let s: String = arg;
            println!("Integer: {}", Solution::length_of_longest_substring(s));
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
