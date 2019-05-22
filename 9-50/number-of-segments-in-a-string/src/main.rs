use std::env;

struct Solution {
}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut result: i32 = 0;
	let mut is_space: bool = true;
	for c in s.chars() {
	    if c != ' ' {
	        if is_space {
		    result += 1;
		}
		is_space = false;
	    } else {
	        is_space = true;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let s: String = arg;
            println!("Count: {}", Solution::count_segments(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameters.");
    }
}
