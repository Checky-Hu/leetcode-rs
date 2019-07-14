use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        // only 2, 3, 5, 7, 11, 13, 17, 19
        let mut result: i32 = 0;
	let mut i: i32 = l;
	while i <= r {
	    let mut bits: i32 = 0;
	    let mut tmp: i32 = i;
	    while tmp > 0 {
	        if tmp & 1 == 1 {
		    bits += 1;
		}
		tmp >>= 1;
	    }
	    match bits {
	        2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => result += 1,
		_ => (),
	    }
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut l: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    1 => l = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let r: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Count: {}", Solution::count_prime_set_bits(l, r));
		break;
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

