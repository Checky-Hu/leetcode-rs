use std::env;

struct Solution {
}

impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        let a_bytes: &[u8] = a.as_bytes();
	let a_len: usize = a_bytes.len();
        let b_bytes: &[u8] = b.as_bytes();
	let b_len: usize = b_bytes.len();
	if a_len != b_len {
	    return false
	} else {
	    if a_len == 0 {
	        return true
	    }
	}

	let mut i: usize = 0;
	let mut j: usize = 0;
	while i < a_len {
	    let mut offset: usize = 0;
	    while i + offset < a_len {
	        if a_bytes[i + offset] == b_bytes[offset] {
		    offset += 1;
		} else {
		    break;
		}
	    }
	    if i + offset == a_len {
	        j = offset;
		break;
	    } else {
	        i += 1;
	    }
	}
	if i == a_len {
	    false
	} else {
	    let mut offset: usize = 0;
	    while j + offset < b_len {
	        if a_bytes[offset] == b_bytes[j + offset] {
		    offset += 1;
		} else {
		    break;
		}
	    }
	    if j + offset == b_len {
	        true
	    } else {
	        false
	    }
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: String = String::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
            1 => a = arg,
	    _ => {
                ret += 1;
                let b: String = arg;
                println!("Is Rotate: {}", Solution::rotate_string(a, b));
		break;
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

