use std::env;

struct Solution {
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
	    return true
	}

        let s_bytes: &[u8] = s.as_bytes();
	let mut i: usize = 0;
	let mut j: usize = s_bytes.len() - 1;
        while i < j {
	    let mut tmp_i: u8 = s_bytes[i];
	    while i < j {
	        match s_bytes[i] {
		    48..=57 | 65..=90 => {
		        tmp_i = s_bytes[i];
		        break;
		    },
		    97..=122 => {
		        tmp_i = s_bytes[i] - 32;
		        break;
		    },
		    _ => i += 1,
		}
	    }

	    let mut tmp_j: u8 = s_bytes[j];
	    while i < j {
	        match s_bytes[j] {
		    48..=57 | 65..=90 => {
		        tmp_j = s_bytes[j];
			break;
		    },
		    97..=122 => {
		        tmp_j = s_bytes[j] - 32;
		        break;
		    },
		    _ => j -= 1,
		}
	    }

	    if i == j {
	        break;
	    } else {
	        if tmp_i == tmp_j {
	            i += 1;
		    j -= 1;
	        } else {
	            return false
	        }
	    }
	}
	true
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let s: String = arg;
	    println!("Palindrome: {}", Solution::is_palindrome(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
