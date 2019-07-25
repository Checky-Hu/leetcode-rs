use std::env;

struct Solution {
}

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        let a_bytes: &[u8] = a.as_bytes();
	let a_len: usize = a_bytes.len();
        let b_bytes: &[u8] = b.as_bytes();
	let b_len: usize = b_bytes.len();
        if a_len != b_len {
	    return false
	}

	let mut diff_1: u8 = 0;
	let mut diff_2: u8 = 0;
	let mut no_equal_count: i32 = 0;
	let mut flags: Vec<i32> = vec![0; 26];
	let mut find_same: bool = false;
	let mut index: usize = 0;
	while index < a_len {
	    if a_bytes[index] != b_bytes[index] {
	        no_equal_count += 1;
	        match no_equal_count {
		    1 => {
		        diff_1 = a_bytes[index];
		        diff_2 = b_bytes[index];
		    },
	            2 => {
		        if diff_1 != b_bytes[index] || diff_2 != a_bytes[index] {
		            return false
		        }
		    },
		    _ => return false,
		}
	    }
	    if no_equal_count == 0 && !find_same {
	        let v: usize = a_bytes[index] as usize - 97;
	        flags[v] += 1;
		if flags[v] == 2 {
		    find_same = true;
		}
	    }
	    index += 1;
	}
	no_equal_count == 2 || (no_equal_count == 0 && find_same)
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
                println!("Buddy: {}", Solution::buddy_strings(a, b));
		break;
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

