use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut result: String = String::new();
        let mut tmp: i32 = n;
	while tmp > 0 {
	    let mut v: i32 = tmp % 26;
	    if v == 0 {
	        v = 26;
	    }
	    result.insert(0, (v as u8 + 64) as char);
	    tmp = (tmp - v) / 26;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    println!("String: {}", Solution::convert_to_title(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
