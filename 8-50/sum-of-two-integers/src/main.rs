use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut tmp: i32 = b;
	let mut result: i32 = a;
	while tmp != 0 {
	    let carry: i32 = (result & tmp & 0x7fffffff) << 1;
	    result ^= tmp;
	    tmp = carry;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => a = i32::from_str(&arg).expect("Error parse."),
	    2 => {
                ret = index;
                let b: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Sum: {}", Solution::get_sum(a, b));
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
