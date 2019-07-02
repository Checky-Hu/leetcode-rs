use std::env;

struct Solution {
}

impl Solution {
    pub fn is_palindrome(bytes: &[u8], start: usize, end: usize) -> bool {
        let mut s: usize = start;
        let mut e: usize = end;
	while s < e {
	    if bytes[s] == bytes[e] {
	        s += 1;
		e -= 1;
	    } else {
	        return false
	    }
	}
	true
    }

    pub fn valid_palindrome(s: String) -> bool {
        let s_bytes: &[u8] = s.as_bytes();
	let mut s: usize = 0;
	let mut e: usize = s_bytes.len() - 1;
	while s < e {
	    if s_bytes[s] == s_bytes[e] {
	        s += 1;
		e -= 1;
	    } else {
	        if Solution::is_palindrome(s_bytes, s + 1, e) ||
		    Solution::is_palindrome(s_bytes, s, e - 1) {
		    return true
		} else {
		    return false
		}
	    }
	}
	true
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
	    ret = 1;
	    let s: String = arg;
            println!("Valid: {}", Solution::valid_palindrome(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
