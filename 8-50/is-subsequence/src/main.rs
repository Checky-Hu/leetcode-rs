use std::env;

struct Solution {
}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
	let mut i: usize = 0;
	let t_bytes: &[u8] = t.as_bytes();
	let len: usize = t_bytes.len();
	for c in s.chars() {
	    while i < len {
	        if t_bytes[i] == c as u8 {
		    break;
		}
		i += 1;
	    }
	    if i == len {
	        return false
	    } else {
	        i += 1;
	    }
	}
	true
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => s = arg,
	    2 => {
                ret = index;
                let t: String = arg;
                println!("Is subsequence: {}", Solution::is_subsequence(s, t));
	        break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
