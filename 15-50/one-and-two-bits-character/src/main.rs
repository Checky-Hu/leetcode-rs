use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let len: usize = bits.len();
        let mut i: usize = 0;
	while i < len - 1 {
	    if bits[i] == 0 {
	        i += 1;
	    } else {
	        i += 2;
	    }
	}
	i == len - 1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut bits: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    _ => {
                ret += 1;
                let bit: i32 = i32::from_str(&arg).expect("Error parse.");
	        bits.push(bit);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Is one bit: {}", Solution::is_one_bit_character(bits));
}

