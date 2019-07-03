use std::env;

struct Solution {
}

impl Solution {
    pub fn is_subarray(a: &[u8], a_len: usize, b: &[u8], b_len: usize) -> bool {
        let mut i: usize = 0;
	while i <= a_len - b_len {
	    let mut j: usize = 0;
	    while j < b_len {
	        if a[i + j] != b[j] {
		    break;
		} else {
		    j += 1;
		}
	    }
	    if j == b_len {
	        return true
	    } else {
	        i += 1;
	    }
	}
	false
    }

    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let a_len: usize = a.len();
        let b_len: usize = b.len();
        let mut s: String = a.clone();
	let mut result: i32 = 1;
	while a_len * (result as usize) < b_len {
	    s.push_str(&a);
	    result += 1;
	}

	if Solution::is_subarray(s.as_bytes(), s.len(), b.as_bytes(), b_len) {
	    return result
	}
	s.push_str(&a);
	result += 1;
	if Solution::is_subarray(s.as_bytes(), s.len(), b.as_bytes(), b_len) {
	    result
	} else {
	    -1
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: String = String::new();
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
	    a = arg;
	} else if 2 == index {
	    ret = 1;
            let b: String = arg;
            println!("Count: {}", Solution::repeated_string_match(a, b));
	    break;
	} else {
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

