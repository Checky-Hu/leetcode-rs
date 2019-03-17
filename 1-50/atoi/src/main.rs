use std::env;

struct Solution {
}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
	let mut result: i64 = 0;
	let mut sign: char = ' ';
	let mut has_no_whitespace: bool = false;
	for c in str.chars() {
	    if c == ' ' {
	        if has_no_whitespace {
		    break;
		}
	    } else if c >= '0' && c <= '9' {
	        has_no_whitespace = true;
	        result = result * 10 + c as i64 - 48;
		if result >= i32::max_value() as i64 {
		    if sign == '-' {
		        if result == i32::max_value() as i64 {
		            return i32::min_value() + 1
			} else {
		            return i32::min_value()
			}
		    } else {
		        return i32::max_value()
		    }
		}
	    } else if c == '+' || c == '-' {
		if has_no_whitespace {
		    break;
		} else {
		    sign = c;
	            has_no_whitespace = true;
		}
	    } else {
	        break;
	    }
	}
	if sign == '-' {
	    (0 - result) as i32
	} else {
	    result as i32
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            println!("Integer: {}", Solution::my_atoi(arg));
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
