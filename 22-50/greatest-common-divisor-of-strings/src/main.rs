use std::env;

struct Solution {
}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.len() < str2.len() {
	    return Solution::gcd_of_strings(str2, str1)
	}

	if str2.is_empty() {
	    return str1
	}

	if !str1.contains(str2.as_str()) {
	    return "".to_string()
	}

	let tmp: String = str1.replace(str2.as_str(), "");
	Solution::gcd_of_strings(tmp, str2)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut str1: String = String::new();
    let mut str2: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            str1 = arg;
        } else if 2 == index {
            ret += 1;
            str2 = arg;
	    break;
        } else {
	}
    }

    if 2 != ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Greatest common divisor: {}", Solution::gcd_of_strings(str1, str2));
}

