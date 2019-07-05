use std::env;

struct Solution {
}

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s_bytes: &[u8] = s.as_bytes();
	let len: usize = s_bytes.len();
	let mut i: usize = 0;
        let mut result: i32 = 0;
	while i < len - 1 {
	    let mut j: usize = i + 1;
	    let mut self_count: usize = 1;
	    while j < len && s_bytes[j] == s_bytes[i] {
	        self_count += 1;
	        j += 1;
	    }
	    if j + self_count <= len {
	        let mut other_count: usize = 0;
	        while s_bytes[j] != s_bytes[i] {
	            other_count += 1;
		    if other_count == self_count {
		        break;
		    } else {
		        j += 1;
		    }
	        }
		result += other_count as i32;
		i += self_count;
	    } else {
	        i += j + self_count - len;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
	    ret = 1;
	    let s: String = arg;
            println!("Count: {}", Solution::count_binary_substrings(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
