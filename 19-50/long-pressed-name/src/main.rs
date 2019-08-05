use std::env;

struct Solution {
}

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name_bytes: &[u8] = name.as_bytes();
	let name_len: usize = name_bytes.len();
        let typed_bytes: &[u8] = typed.as_bytes();
	let typed_len: usize = typed_bytes.len();
	if name_len > typed_len {
	    return false
	}

	let mut i: usize = 0;
	let mut j: usize = 0;
	while i < name_len {
	    if j == typed_len {
	        return false
	    } else if name_bytes[i] == typed_bytes[j] {
	        i += 1;
		j += 1;
	    } else {
	        if i > 0 && name_bytes[i - 1] == typed_bytes[j] {
		    j += 1;
		} else {
		    return false
		}
	    }
	}
	while j < typed_len {
	    if name_bytes[i - 1] == typed_bytes[j] {
	        j += 1;
	    } else {
	        return false
	    }
	}
	true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut name: String = String::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
            1 => name = arg,
	    _ => {
                ret += 1;
                let typed: String = arg;
                println!("Long pressed: {}", Solution::is_long_pressed_name(name, typed));
		break;
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

