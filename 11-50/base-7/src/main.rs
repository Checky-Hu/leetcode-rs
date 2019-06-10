use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut result: String = String::new();
	let mut tmp: i32 = num;
	if tmp < 0 {
	    tmp = 0 - tmp;
	    result.push('-');
	}

	let mut base_7: i32 = 7;
	while tmp >= base_7 {
	    base_7 *= 7;
	}
	base_7 /= 7;
	while base_7 > 0 {
	    let n: i32 = tmp / base_7;
	    tmp -= n * base_7;
	    base_7 /= 7;
	    result.push((n as u8 + 48) as char);
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Base 7: {}", Solution::convert_to_base7(num));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
