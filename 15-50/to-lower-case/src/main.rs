use std::env;

struct Solution {
}

impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut result: String = String::new();
	for b in str.bytes() {
	    if b >= 65 && b <= 90 {
	        result.push((b + 32) as char);
	    } else {
	        result.push(b as char);
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
	    let str: String = arg;
            println!("Lower case: {}", Solution::to_lower_case(str));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
