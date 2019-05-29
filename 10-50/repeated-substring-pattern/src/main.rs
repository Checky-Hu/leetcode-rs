use std::env;

struct Solution {
}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s_bytes: &[u8] = s.as_bytes();
        let len: usize = s_bytes.len();
	let mut distance: usize = len / 2;
	while distance > 0 {
	    if len % distance == 0 {
	        let mut i: usize = 0;
		while i < distance {
		    let mut j: usize = i;
		    while j + distance < len && s_bytes[j] == s_bytes[j + distance] {
		        j += distance;
		    }
		    if j + distance < len {
		        break;
		    } else {
		        i += 1;
		    }
		}
		if i == distance {
		    return true
		}
	    }
	    distance -= 1;
	}
        false
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let s: String = arg;
            println!("Repeated: {}", Solution::repeated_substring_pattern(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
