use std::env;

struct Solution {
}

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut result: i32 = 0;
	for c in s.chars() {
	    result = result * 26 + (c as u8 - 64) as i32;
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
	    println!("String: {}", Solution::title_to_number(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
