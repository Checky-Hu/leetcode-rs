use std::env;

struct Solution {
}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut result: i32 = 0;

	let s_bytes: &[u8] = s.as_bytes();
	let s_len: usize = s.len();
	let mut offset: usize = 1;
	let mut is_char: bool = false;
	while offset <= s_len {
	    if s_bytes[s_len - offset] == ' ' as u8 {
	        if is_char {
		    return result
		}
	    } else {
	        is_char = true;
		result += 1;
	    }
	    offset += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
            ret = index;
	    s = arg;
	    break;
	} else {
	    continue;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Length: {}", Solution::length_of_last_word(s));
}
