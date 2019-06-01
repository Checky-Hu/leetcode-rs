use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut xor: i32 = x ^ y;
	let mut result: i32 = 0;
	while xor > 0 {
	    result += xor & 1;
	    xor >>= 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut x: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => x = i32::from_str(&arg).expect("Error parse."),
	    2 => {
                ret += 1;
                let y: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Distance: {}", Solution::hamming_distance(x, y));
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
