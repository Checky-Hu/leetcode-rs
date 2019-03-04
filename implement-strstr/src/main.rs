use std::env;

struct Solution {
}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let src_bytes = haystack.as_bytes();
	let src_len: usize = src_bytes.len();
        let dst_bytes = needle.as_bytes();
	let dst_len: usize = dst_bytes.len();
	if dst_len == 0 {
	    0
	} else if src_len < dst_len {
	    -1
	} else {
            let mut index: usize = 0;
	    while index <= src_len - dst_len {
	        let mut tmp_i: usize = 0;
	        while tmp_i < dst_len {
	            if src_bytes[index + tmp_i] != dst_bytes[tmp_i] {
		        break;
		    }
		    tmp_i += 1;
	        }

	        if tmp_i == dst_len {
	            return index as i32;
	        } else {
	            index += 1;
	        }
	    }
	    -1
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut haystack: String = String::new();
    let mut needle: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
	    haystack = arg;
	} else if index == 2 {
	    needle = arg;
	    ret = index;
	    break;
	} else {
	    continue;
	}
    }

    if ret == 0 {
        println!("Require at least two parameters.");
	return;
    }
    println!("Index: {}", Solution::str_str(haystack, needle));
}
